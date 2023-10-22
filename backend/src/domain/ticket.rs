use crate::auth::Authenticated;
use crate::domain::group::{Group, GroupId, GroupView};
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::id::Id;
use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use cqrs_es::persist::{ViewContext, ViewRepository};
use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, Query, View};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use ts_rs::TS;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketId(pub Id);

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct CreateTicket {
    pub destination: TicketDestination,
    pub title: String,
    pub body: String,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct SendTicketMessage {
    pub body: String,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct ChangeStatus {
    pub new_status: TicketStatus,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct ChangeAssignee {
    pub new_assignee: Option<UserId>,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
#[serde(tag = "type")]
pub enum TicketCommand {
    Create(CreateTicket),
    SendTicketMessage(SendTicketMessage),
    ChangeStatus(ChangeStatus),
    ChangeAssignee(ChangeAssignee),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketEvent {
    Create {
        destination: TicketDestination,
        owner: UserId,
        title: String,
    },
    Message {
        date: DateTime<Utc>,
        from: UserId,
        text: String,
    },
    StatusChange {
        date: DateTime<Utc>,
        old_status: TicketStatus,
        new_status: TicketStatus,
    },
    AssigneeChange {
        date: DateTime<Utc>,
        old_assignee: Option<UserId>,
        new_assignee: Option<UserId>,
    },
}

impl DomainEvent for TicketEvent {
    fn event_type(&self) -> String {
        match self {
            TicketEvent::Create { .. } => "Create".to_string(),
            TicketEvent::Message { .. } => "Message".to_string(),
            TicketEvent::StatusChange { .. } => "StatusChange".to_string(),
            TicketEvent::AssigneeChange { .. } => "AssigneeChange".to_string(),
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

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, TS, Serialize, Deserialize)]
#[ts(export)]
pub enum TicketStatus {
    #[default]
    Pending,
    InProgress,
    Declined,
    Fixed,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TS, Serialize, Deserialize)]
#[ts(export)]
pub enum TicketDestination {
    User(UserId),
    Group(GroupId),
}

#[derive(Debug, Clone, Eq, PartialEq, TS, Serialize, Deserialize)]
#[ts(export)]
#[serde(tag = "type")]
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

#[derive(Debug, Clone, Eq, PartialEq, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketTimelineItem {
    #[ts(type = "string")]
    date: DateTime<Utc>,
    content: TicketTimelineItemContent,
}

pub struct TicketServices {
    pub group_view_repository: Arc<dyn ViewRepository<GroupView, Group>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketContent {
    pub destination: TicketDestination,
    pub owner: UserId,
    pub assignee: Option<UserId>,
    pub title: String,
    pub status: TicketStatus,
}

impl TicketContent {
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
                else {
                    error!("Group not found");
                    return Err(TicketError::Forbidden);
                };
                let group = group.unwrap();
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Ticket {
    #[default]
    NotCreated,
    Created(TicketContent),
}

impl Ticket {
    fn unwrap_ref(&self) -> &TicketContent {
        match self {
            Ticket::NotCreated => panic!("Ticket not created"),
            Ticket::Created(content) => content,
        }
    }

    fn unwrap_mut(&mut self) -> &mut TicketContent {
        match self {
            Ticket::NotCreated => panic!("Ticket not created"),
            Ticket::Created(content) => content,
        }
    }
}

#[async_trait]
impl Aggregate for Ticket {
    type Command = Authenticated<TicketCommand>;
    type Event = TicketEvent;
    type Error = TicketError;
    type Services = TicketServices;

    fn aggregate_type() -> String {
        "Ticket".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command.payload {
            TicketCommand::Create(CreateTicket {
                destination,
                title,
                body,
            }) => {
                let Ticket::NotCreated = self else {
                    return Err(TicketError::AlreadyExists);
                };
                Ok(vec![
                    TicketEvent::Create {
                        destination,
                        owner: command.user_id,
                        title,
                    },
                    TicketEvent::Message {
                        // TODO: make this external maybe? Unit testing is hard otherwise...
                        date: Utc::now(),
                        from: command.user_id,
                        text: body,
                    },
                ])
            }
            TicketCommand::SendTicketMessage(SendTicketMessage { body }) => {
                let _this = self.unwrap_ref();
                Ok(vec![TicketEvent::Message {
                    date: Utc::now(),
                    from: command.user_id,
                    text: body,
                }])
            }
            TicketCommand::ChangeStatus(ChangeStatus { new_status }) => {
                let this = self.unwrap_ref();
                this.check_access(command.user_id, service).await?;
                Ok(vec![TicketEvent::StatusChange {
                    date: Utc::now(),
                    old_status: this.status,
                    new_status,
                }])
            }
            TicketCommand::ChangeAssignee(ChangeAssignee { new_assignee }) => {
                let this = self.unwrap_ref();
                this.check_access(command.user_id, service).await?;
                Ok(vec![TicketEvent::AssigneeChange {
                    date: Utc::now(),
                    old_assignee: this.assignee,
                    new_assignee,
                }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TicketEvent::Create {
                destination,
                owner,
                title,
            } => {
                let Ticket::NotCreated = self else {
                    panic!("Ticket already created");
                };
                *self = Ticket::Created(TicketContent {
                    destination,
                    owner,
                    assignee: None,
                    title,
                    status: TicketStatus::Pending,
                });
            }
            TicketEvent::Message {
                date: _,
                from: _,
                text: _,
            } => {
                let _this = self.unwrap_mut();
            }
            TicketEvent::StatusChange {
                date: _,
                old_status: _,
                new_status,
            } => {
                let this = self.unwrap_mut();
                this.status = new_status;
            }
            TicketEvent::AssigneeChange {
                date: _,
                old_assignee: _,
                new_assignee,
            } => {
                let this = self.unwrap_mut();
                this.assignee = new_assignee;
            }
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TicketView {
    #[default]
    NotCreated,
    Created(TicketViewContent),
}

impl TicketView {
    pub fn unwrap(self) -> TicketViewContent {
        match self {
            TicketView::NotCreated => panic!("Ticket not created"),
            TicketView::Created(content) => content,
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut TicketViewContent {
        match self {
            TicketView::NotCreated => panic!("Ticket not created"),
            TicketView::Created(content) => content,
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketViewContent {
    pub id: TicketId,
    pub destination: TicketDestination,
    pub owner: UserId,
    pub assignee: Option<UserId>,
    pub title: String,
    pub status: TicketStatus,
    pub timeline: Vec<TicketTimelineItem>,
}

impl View<Ticket> for TicketView {
    fn update(&mut self, event: &EventEnvelope<Ticket>) {
        match event.payload {
            TicketEvent::Create {
                destination,
                owner,
                ref title,
            } => {
                let TicketView::NotCreated = self else {
                    panic!("Ticket already created");
                };

                *self = TicketView::Created(TicketViewContent {
                    id: TicketId(Id::from_str(&event.aggregate_id).unwrap()),
                    destination,
                    owner,
                    assignee: None,
                    title: title.clone(),
                    status: TicketStatus::Pending,
                    timeline: vec![],
                });
            }
            TicketEvent::Message {
                date,
                from,
                ref text,
            } => {
                let this = self.unwrap_mut();
                this.timeline.push(TicketTimelineItem {
                    date,
                    content: TicketTimelineItemContent::Message {
                        from,
                        text: text.clone(),
                    },
                });
            }
            TicketEvent::StatusChange {
                date,
                old_status,
                new_status,
            } => {
                let this = self.unwrap_mut();
                this.status = new_status;
                this.timeline.push(TicketTimelineItem {
                    date,
                    content: TicketTimelineItemContent::StatusChange {
                        old: old_status,
                        new: new_status,
                    },
                });
            }
            TicketEvent::AssigneeChange {
                date,
                old_assignee,
                new_assignee,
            } => {
                let this = self.unwrap_mut();
                this.assignee = new_assignee;
                this.timeline.push(TicketTimelineItem {
                    date,
                    content: TicketTimelineItemContent::AssigneeChange {
                        old: old_assignee,
                        new: new_assignee,
                    },
                });
            }
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketListingViewExpandedItem {
    pub id: TicketId,
    pub destination: TicketDestination,
    pub owner: UserId,
    pub assignee: Option<UserId>,
    pub title: String,
    pub status: TicketStatus,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TicketListingView {
    pub items: HashSet<TicketId>,
}

impl View<Ticket> for TicketListingView {
    fn update(&mut self, _event: &EventEnvelope<Ticket>) {
        // actually only used in GenericQuery
        unreachable!()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TicketListingKind {
    Owned,
    Assigned,
}

pub struct TicketListingQuery<R>
where
    R: ViewRepository<TicketListingView, Ticket>,
{
    listing_view_repository: Arc<R>,
    kind: TicketListingKind,
}

impl<R> TicketListingQuery<R>
where
    R: ViewRepository<TicketListingView, Ticket>,
{
    pub fn new(view_repository: Arc<R>, kind: TicketListingKind) -> Self {
        Self {
            listing_view_repository: view_repository,
            kind,
        }
    }
}

#[async_trait]
impl<R> Query<Ticket> for TicketListingQuery<R>
where
    R: ViewRepository<TicketListingView, Ticket>,
{
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Ticket>]) {
        for event in events {
            match (self.kind, &event.payload) {
                (TicketListingKind::Owned, TicketEvent::Create { owner, .. }) => {
                    let user_id = owner.0.to_string();

                    let (mut view, context) = self
                        .listing_view_repository
                        .load_with_context(&user_id)
                        .await
                        .unwrap()
                        .unwrap_or_else(|| {
                            (TicketListingView::default(), ViewContext::new(user_id, 0))
                        });

                    view.items
                        .insert(TicketId(Id::from_str(&event.aggregate_id).unwrap()));
                    self.listing_view_repository
                        .update_view(view, context)
                        .await
                        .unwrap();
                }
                (
                    TicketListingKind::Assigned,
                    TicketEvent::AssigneeChange {
                        old_assignee,
                        new_assignee,
                        ..
                    },
                ) => {
                    if let Some(old_assignee) = old_assignee {
                        let user_id = old_assignee.0.to_string();

                        let (mut view, context) = self
                            .listing_view_repository
                            .load_with_context(&user_id)
                            .await
                            .unwrap()
                            .unwrap_or_else(|| {
                                (TicketListingView::default(), ViewContext::new(user_id, 0))
                            });

                        view.items
                            .remove(&TicketId(Id::from_str(&event.aggregate_id).unwrap()));
                        self.listing_view_repository
                            .update_view(view, context)
                            .await
                            .unwrap();
                    }

                    if let Some(new_assignee) = new_assignee {
                        let user_id = new_assignee.0.to_string();

                        let (mut view, context) = self
                            .listing_view_repository
                            .load_with_context(&user_id)
                            .await
                            .unwrap()
                            .unwrap_or_else(|| {
                                (TicketListingView::default(), ViewContext::new(user_id, 0))
                            });

                        view.items
                            .insert(TicketId(Id::from_str(&event.aggregate_id).unwrap()));
                        self.listing_view_repository
                            .update_view(view, context)
                            .await
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
