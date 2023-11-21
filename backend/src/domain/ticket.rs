use crate::auth::Authenticated;
use crate::domain::group::{GroupId, GroupView};
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::related_data::CollectIds;
use crate::view_repositry_ext::ViewRepositoryExt;
use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use cqrs_es::lifecycle::{
    CreateEnvelope, LifecycleAggregate, LifecycleAggregateState, LifecycleEnvelope, LifecycleEvent,
    LifecycleView, LifecycleViewState, UpdateEnvelope,
};
use cqrs_es::persist::ViewRepository;
use cqrs_es::{AnyId, Id};
use cqrs_es::{DomainEvent, Query, View};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tracing::error;
use ts_rs::TS;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketId(pub Id);

impl CollectIds<UserId> for TicketId {
    fn collect_ids(&self, _: &mut IndexSet<UserId>) {}
}
impl CollectIds<GroupId> for TicketId {
    fn collect_ids(&self, _: &mut IndexSet<GroupId>) {}
}

impl AnyId for TicketId {
    fn from_id(id: Id) -> Self {
        Self(id)
    }

    fn id(&self) -> Id {
        self.0
    }
}

#[derive(Debug, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub struct CreateTicket {
    pub destination: TicketDestination,
    pub title: String,
    pub body: String,
}

#[derive(Debug, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub struct SendTicketMessage {
    pub body: String,
}

#[derive(Debug, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub struct ChangeStatus {
    pub new_status: TicketStatus,
}

#[derive(Debug, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub struct ChangeAssignee {
    pub new_assignee: Option<UserId>,
}

#[derive(Debug, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[serde(tag = "type")]
#[collect_ids(UserId, GroupId)]
pub enum UpdateTicket {
    SendTicketMessage(SendTicketMessage),
    ChangeStatus(ChangeStatus),
    ChangeAssignee(ChangeAssignee),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketCreated {
    date: DateTime<Utc>,
    destination: TicketDestination,
    owner: UserId,
    title: String,
}

impl DomainEvent for TicketCreated {
    fn event_type(&self) -> String {
        "".to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketUpdated {
    Message {
        date: DateTime<Utc>,
        from: UserId,
        text: String,
    },
    StatusChanged {
        date: DateTime<Utc>,
        old_status: TicketStatus,
        new_status: TicketStatus,
    },
    AssigneeChanged {
        date: DateTime<Utc>,
        old_assignee: Option<UserId>,
        new_assignee: Option<UserId>,
    },
}

impl DomainEvent for TicketUpdated {
    fn event_type(&self) -> String {
        match self {
            TicketUpdated::Message { .. } => "Message".to_string(),
            TicketUpdated::StatusChanged { .. } => "StatusChanged".to_string(),
            TicketUpdated::AssigneeChanged { .. } => "AssigneeChanged".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Snafu, Debug)]
pub enum TicketError {
    /// Ticket with the specified ID already exists
    AlreadyExists,
    /// User cannot perform this action with this ticket
    Forbidden,
}

impl ApiError for TicketError {
    fn status_code(&self) -> StatusCode {
        match self {
            TicketError::AlreadyExists => StatusCode::BAD_REQUEST,
            TicketError::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub enum TicketStatus {
    #[default]
    Pending,
    InProgress,
    Declined,
    Fixed,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TS, Serialize, Deserialize, CollectIds)]
#[serde(tag = "type", content = "id")]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub enum TicketDestination {
    User(UserId),
    Group(GroupId),
}

impl Display for TicketDestination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketDestination::User(user) => write!(f, "user-{}", user.0),
            TicketDestination::Group(group) => write!(f, "group-{}", group.0),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[serde(tag = "type")]
#[collect_ids(UserId, GroupId)]
pub enum TicketTimelineItemContent {
    Message {
        from: UserId,
        text: String,
    },
    StatusChange {
        old: TicketStatus,
        new: TicketStatus,
    },
    AssigneeChange {
        old: Option<UserId>,
        new: Option<UserId>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub struct TicketTimelineItem {
    date: DateTime<Utc>,
    content: TicketTimelineItemContent,
}

pub struct TicketServices {
    pub group_view_repository: Arc<dyn ViewRepository<LifecycleViewState<GroupView>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub destination: TicketDestination,
    pub owner: UserId,
    pub assignee: Option<UserId>,
    pub title: String,
    pub status: TicketStatus,
}

pub type TicketAggregate = LifecycleAggregateState<Ticket>;

impl Ticket {
    pub async fn check_access(
        &self,
        user: UserId,
        services: &TicketServices,
    ) -> Result<(), TicketError> {
        match self.destination {
            TicketDestination::User(dest_user) => {
                if user == dest_user {
                    Ok(())
                } else {
                    error!("User does not have access to this ticket because it is not addressed to them");
                    Err(TicketError::Forbidden)
                }
            }
            TicketDestination::Group(group) => {
                let Some(group) = services
                    .group_view_repository
                    .load(&group.0.to_string())
                    .await
                    .unwrap()
                    .and_then(|v| v.into_created())
                else {
                    error!("Group not found");
                    return Err(TicketError::Forbidden);
                };
                if group.members.contains(&user) {
                    Ok(())
                } else {
                    error!("User does not have access to this ticket because they are not a member of the group it is addressed to");
                    Err(TicketError::Forbidden)
                }
            }
        }
    }
}

#[async_trait]
impl LifecycleAggregate for Ticket {
    type Id = TicketId;
    type CreateCommand = Authenticated<CreateTicket>;
    type UpdateCommand = Authenticated<UpdateTicket>;
    type DeleteCommand = ();
    type CreateEvent = TicketCreated;
    type UpdateEvent = TicketUpdated;
    type Error = TicketError;
    type Services = TicketServices;

    fn aggregate_type() -> String {
        "Ticket".to_string()
    }

    async fn handle_create(
        Authenticated {
            user_id,
            payload:
                CreateTicket {
                    title,
                    destination,
                    body,
                },
        }: Self::CreateCommand,
        _service: &Self::Services,
    ) -> Result<(Self::CreateEvent, Vec<Self::UpdateEvent>), Self::Error> {
        let created = TicketCreated {
            // TODO: make this external maybe? Unit testing is hard otherwise...
            date: Utc::now(),
            destination,
            owner: user_id,
            title,
        };

        let mut updated = vec![TicketUpdated::Message {
            // TODO: make this external maybe? Unit testing is hard otherwise...
            date: Utc::now(),
            from: user_id,
            text: body,
        }];
        if let TicketDestination::User(dest) = destination {
            updated.push(TicketUpdated::AssigneeChanged {
                date: Utc::now(),
                old_assignee: None,
                new_assignee: Some(dest),
            });
        };

        Ok((created, updated))
    }

    async fn handle(
        &self,
        Authenticated {
            user_id,
            payload: command,
        }: Self::UpdateCommand,
        service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error> {
        let mut events = Vec::new();

        match command {
            UpdateTicket::SendTicketMessage(SendTicketMessage { body }) => {
                events.push(TicketUpdated::Message {
                    date: Utc::now(),
                    from: user_id,
                    text: body,
                });
            }
            UpdateTicket::ChangeStatus(ChangeStatus { new_status }) => {
                self.check_access(user_id, service).await?;
                if self.status != new_status {
                    events.push(TicketUpdated::StatusChanged {
                        date: Utc::now(),
                        old_status: self.status,
                        new_status,
                    })
                }
            }
            UpdateTicket::ChangeAssignee(ChangeAssignee { new_assignee }) => {
                self.check_access(user_id, service).await?;
                if self.assignee != new_assignee {
                    events.push(TicketUpdated::AssigneeChanged {
                        date: Utc::now(),
                        old_assignee: self.assignee,
                        new_assignee,
                    });
                }
            }
        }

        Ok(events)
    }

    async fn handle_delete(
        &self,
        _command: Self::DeleteCommand,
        _service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error> {
        todo!()
    }

    fn apply_create(
        TicketCreated {
            date: _,
            title,
            destination,
            owner,
        }: Self::CreateEvent,
    ) -> Self {
        Self {
            destination,
            owner,
            assignee: None,
            title,
            status: TicketStatus::Pending,
        }
    }

    fn apply(&mut self, event: Self::UpdateEvent) {
        match event {
            TicketUpdated::Message {
                date: _,
                from: _,
                text: _,
            } => {}
            TicketUpdated::StatusChanged {
                date: _,
                old_status: _,
                new_status,
            } => {
                self.status = new_status;
            }
            TicketUpdated::AssigneeChanged {
                date: _,
                old_assignee: _,
                new_assignee,
            } => {
                self.assignee = new_assignee;
            }
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub struct TicketView {
    pub id: TicketId,
    pub destination: TicketDestination,
    pub owner: UserId,
    pub assignee: Option<UserId>,
    pub title: String,
    pub status: TicketStatus,
    pub timeline: Vec<TicketTimelineItem>,
    pub latest_update: DateTime<Utc>,
}

impl LifecycleView for TicketView {
    type Aggregate = Ticket;
    fn create(event: CreateEnvelope<'_, Self::Aggregate>) -> Self {
        let TicketCreated {
            date,
            destination,
            owner,
            ref title,
        } = *event.payload;

        TicketView {
            id: event.aggregate_id,
            destination,
            owner,
            assignee: None,
            title: title.clone(),
            status: TicketStatus::Pending,
            timeline: vec![],
            latest_update: date,
        }
    }

    fn update(&mut self, event: UpdateEnvelope<'_, Self::Aggregate>) {
        match *event.payload {
            TicketUpdated::Message {
                date,
                from,
                ref text,
            } => {
                self.timeline.push(TicketTimelineItem {
                    date,
                    content: TicketTimelineItemContent::Message {
                        from,
                        text: text.clone(),
                    },
                });
                self.latest_update = date;
            }
            TicketUpdated::StatusChanged {
                date,
                old_status,
                new_status,
            } => {
                self.status = new_status;
                self.timeline.push(TicketTimelineItem {
                    date,
                    content: TicketTimelineItemContent::StatusChange {
                        old: old_status,
                        new: new_status,
                    },
                });
                self.latest_update = date;
            }
            TicketUpdated::AssigneeChanged {
                date,
                old_assignee,
                new_assignee,
            } => {
                self.assignee = new_assignee;
                self.timeline.push(TicketTimelineItem {
                    date,
                    content: TicketTimelineItemContent::AssigneeChange {
                        old: old_assignee,
                        new: new_assignee,
                    },
                });
                self.latest_update = date;
            }
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize, CollectIds)]
#[ts(export)]
#[collect_ids(UserId, GroupId)]
pub struct TicketListingViewExpandedItem {
    pub id: TicketId,
    pub destination: TicketDestination,
    pub owner: UserId,
    pub assignee: Option<UserId>,
    pub title: String,
    pub status: TicketStatus,
    pub latest_update: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TicketListingView {
    pub items: HashSet<TicketId>,
}

impl View for TicketListingView {
    type Aggregate = TicketAggregate;
}

#[derive(Copy, Clone, Debug)]
pub enum TicketListingKind {
    Owned,
    Assigned,
    Destination,
}

pub struct TicketListingQuery<R>
where
    R: ViewRepository<TicketListingView>,
{
    listing_view_repository: Arc<R>,
    kind: TicketListingKind,
}

impl<R> TicketListingQuery<R>
where
    R: ViewRepository<TicketListingView>,
{
    pub fn new(view_repository: Arc<R>, kind: TicketListingKind) -> Self {
        Self {
            listing_view_repository: view_repository,
            kind,
        }
    }
}

#[async_trait]
impl<R> Query<TicketAggregate> for TicketListingQuery<R>
where
    R: ViewRepository<TicketListingView>,
{
    async fn dispatch(&self, _aggregate_id: TicketId, events: &[LifecycleEnvelope<Ticket>]) {
        for event in events {
            match (self.kind, &event.payload) {
                (
                    TicketListingKind::Owned,
                    LifecycleEvent::Created(TicketCreated { owner, .. }),
                ) => {
                    let user_id = owner.0.to_string();

                    self.listing_view_repository
                        .load_modify_update_default(&user_id, |view| {
                            view.items.insert(event.aggregate_id);
                        })
                        .await
                        .expect("Persistence error");
                }
                (
                    TicketListingKind::Assigned,
                    LifecycleEvent::Updated(TicketUpdated::AssigneeChanged {
                        old_assignee,
                        new_assignee,
                        ..
                    }),
                ) => {
                    if let Some(old_assignee) = old_assignee {
                        let user_id = old_assignee.0.to_string();

                        self.listing_view_repository
                            .load_modify_update_default(&user_id, |view| {
                                view.items.remove(&event.aggregate_id);
                            })
                            .await
                            .expect("Persistence error");
                    }

                    if let Some(new_assignee) = new_assignee {
                        let user_id = new_assignee.0.to_string();

                        self.listing_view_repository
                            .load_modify_update_default(&user_id, |view| {
                                view.items.insert(event.aggregate_id);
                            })
                            .await
                            .expect("Persistence error");
                    }
                }
                (
                    TicketListingKind::Destination,
                    LifecycleEvent::Created(TicketCreated { destination, .. }),
                ) => {
                    let dest_id = destination.to_string();

                    self.listing_view_repository
                        .load_modify_update_default(&dest_id, |view| {
                            view.items.insert(event.aggregate_id);
                        })
                        .await
                        .expect("Persistence error");
                }
                _ => {}
            }
        }
    }
}
