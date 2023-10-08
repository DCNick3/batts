use crate::auth::Authenticated;
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::id::Id;
use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, View};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::str::FromStr;
use ts_rs::TS;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct TicketId(pub Id);

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct CreateTicket {
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
#[serde(tag = "type")]
pub enum TicketCommand {
    Create(CreateTicket),
    SendTicketMessage(SendTicketMessage),
    // TODO: actually, only admins should be able to change status
    ChangeStatus(TicketStatus),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketEvent {
    Create {
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
            TicketCommand::Create(CreateTicket { title, body }) => {
                let Ticket::NotCreated = self else {
                    return Err(TicketError::AlreadyExists);
                };
                Ok(vec![
                    TicketEvent::Create {
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
            TicketCommand::ChangeStatus(new_status) => {
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
            TicketEvent::Create { owner, title } => {
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
    pub owner: UserId,
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
            TicketEvent::Create { owner, ref title } => {
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
