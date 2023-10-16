use crate::auth::Authenticated;
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
use ts_rs::TS;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, TS, Serialize, Deserialize)]
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
#[serde(tag = "type")]
pub enum TicketCommand {
    Create(CreateTicket),
    SendTicketMessage(SendTicketMessage),
    // TODO: actually, only admins should be able to change status
    ChangeStatus(ChangeStatus),
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
        new_status: TicketStatus,
    },
}

impl DomainEvent for TicketEvent {
    fn event_type(&self) -> String {
        match self {
            TicketEvent::Create { .. } => "Create".to_string(),
            TicketEvent::Message { .. } => "Message".to_string(),
            TicketEvent::StatusChange { .. } => "StatusChange".to_string(),
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
}

impl ApiError for TicketError {
    fn status_code(&self) -> StatusCode {
        match self {
            TicketError::AlreadyExists => StatusCode::BAD_REQUEST,
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

// TODO: this is temporary until we have a proper queue system
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, TS, Serialize, Deserialize)]
#[ts(export)]
pub enum TicketDestination {
    #[default]
    ItDepartment,
    DormManager,
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
}

#[derive(Debug, Clone, Eq, PartialEq, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketTimelineItem {
    #[ts(type = "string")]
    date: DateTime<Utc>,
    content: TicketTimelineItemContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketContent {
    pub owner: UserId,
    pub title: String,
    pub status: TicketStatus,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Ticket {
    #[default]
    NotCreated,
    Created(TicketContent),
}

#[async_trait]
impl Aggregate for Ticket {
    type Command = Authenticated<TicketCommand>;
    type Event = TicketEvent;
    type Error = TicketError;
    type Services = ();

    fn aggregate_type() -> String {
        "Ticket".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
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
                Ok(vec![TicketEvent::Message {
                    date: Utc::now(),
                    from: command.user_id,
                    text: body,
                }])
            }
            TicketCommand::ChangeStatus(ChangeStatus { new_status }) => {
                let Ticket::Created(_content) = self else {
                    panic!("Ticket not created");
                };
                Ok(vec![TicketEvent::StatusChange {
                    date: Utc::now(),
                    new_status,
                }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TicketEvent::Create {
                destination: _,
                owner,
                title,
            } => {
                let Ticket::NotCreated = self else {
                    panic!("Ticket already created");
                };
                *self = Ticket::Created(TicketContent {
                    owner,
                    title,
                    status: TicketStatus::Pending,
                });
            }
            TicketEvent::Message {
                date: _,
                from: _,
                text: _,
            } => {
                let Ticket::Created(_content) = self else {
                    panic!("Ticket not created");
                };
            }
            TicketEvent::StatusChange {
                date: _,
                new_status,
            } => {
                let Ticket::Created(content) = self else {
                    panic!("Ticket not created");
                };
                content.status = new_status;
            }
        }
    }
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketView {
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
        if self.id.0.is_default() {
            self.id = TicketId(Id::from_str(&event.aggregate_id).unwrap());
        }

        match event.payload {
            TicketEvent::Create {
                destination,
                owner,
                ref title,
            } => {
                self.destination = destination;
                self.owner = owner;
                self.title = title.clone();
                self.status = TicketStatus::Pending;
            }
            TicketEvent::Message {
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
            }
            TicketEvent::StatusChange { date, new_status } => {
                let old_status = self.status;
                self.status = new_status;
                self.timeline.push(TicketTimelineItem {
                    date,
                    content: TicketTimelineItemContent::StatusChange {
                        old: old_status,
                        new: new_status,
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
                _ => {}
            }
        }
    }
}
