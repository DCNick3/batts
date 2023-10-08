use crate::auth::Authenticated;
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::id::Id;
use async_trait::async_trait;
use axum::http::StatusCode;
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
#[serde(tag = "type")]
pub enum TicketCommand {
    Create(CreateTicket),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketEvent {
    Create { owner: UserId, title: String },
    Message { text: String },
}

impl DomainEvent for TicketEvent {
    fn event_type(&self) -> String {
        match self {
            TicketEvent::Create { .. } => "Create".to_string(),
            TicketEvent::Message { .. } => "Message".to_string(),
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TicketContent {
    pub owner: UserId,
    pub title: String,
    pub messages: Vec<String>,
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
                    TicketEvent::Message { text: body },
                ])
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
                    messages: vec![],
                });
            }
            TicketEvent::Message { text } => {
                let Ticket::Created(content) = self else {
                    panic!("Ticket not created");
                };
                content.messages.push(text);
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
    pub messages: Vec<String>,
}

impl View<Ticket> for TicketView {
    fn update(&mut self, event: &EventEnvelope<Ticket>) {
        if self.id.0.is_default() {
            self.id = TicketId(Id::from_str(&event.aggregate_id).unwrap());
        }

        match &event.payload {
            TicketEvent::Create { owner, title } => {
                self.owner = *owner;
                self.title = title.clone();
            }
            TicketEvent::Message { text } => {
                self.messages.push(text.clone());
            }
        }
    }
}
