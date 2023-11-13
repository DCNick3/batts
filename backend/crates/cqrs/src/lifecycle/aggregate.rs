use crate::{Aggregate, AnyId, DomainEvent};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[async_trait]
pub trait LifecycleAggregate: Serialize + DeserializeOwned + Sync + Send {
    /// Specifies the id type of the aggregate
    type Id: AnyId;

    type CreateCommand: Debug + Sync + Send;
    /// Specifies the inbound command used to make changes in the state of the Aggregate.
    type UpdateCommand: Debug + Sync + Send;
    type DeleteCommand: Debug + Sync + Send;

    type CreateEvent: DomainEvent;
    /// Specifies the published events representing some change in state of the Aggregate.
    type UpdateEvent: DomainEvent;
    /// The error returned when a command fails due to business logic.
    /// This is used to provide feedback to the user as to the nature of why the command was refused.

    type Error: std::error::Error;
    /// The external services required for the logic within the Aggregate
    type Services: Send + Sync;

    fn aggregate_type() -> String;

    async fn handle_create(
        command: Self::CreateCommand,
        service: &Self::Services,
    ) -> Result<(Self::CreateEvent, Vec<Self::UpdateEvent>), Self::Error>;
    async fn handle(
        &self,
        command: Self::UpdateCommand,
        service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error>;
    async fn handle_delete(
        &self,
        command: Self::DeleteCommand,
        service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error>;

    fn apply_create(event: Self::CreateEvent) -> Self;
    fn apply(&mut self, event: Self::UpdateEvent);
}

pub enum LifecycleCommand<A: LifecycleAggregate> {
    Create(A::CreateCommand),
    Update(A::UpdateCommand),
    Delete(A::DeleteCommand),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
pub enum LifecycleEvent<CreateEvent: DomainEvent, UpdateEvent: DomainEvent> {
    Created(CreateEvent),
    Updated(UpdateEvent),
    Deleted,
}

impl<CreateEvent: DomainEvent, UpdateEvent: DomainEvent> DomainEvent
    for LifecycleEvent<CreateEvent, UpdateEvent>
{
    fn event_type(&self) -> String {
        match self {
            LifecycleEvent::Created(inner) => format!("Create({})", inner.event_type()),
            LifecycleEvent::Updated(inner) => format!("Update({})", inner.event_type()),
            LifecycleEvent::Deleted => "Delete".to_string(),
        }
    }

    fn event_version(&self) -> String {
        match self {
            LifecycleEvent::Created(inner) => inner.event_version(),
            LifecycleEvent::Updated(inner) => inner.event_version(),
            LifecycleEvent::Deleted => "0".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum LifecycleError<E> {
    #[error("The object wasn't created yet")]
    NotCreated,
    #[error("The object was already created")]
    AlreadyCreated,
    #[error("The object was already deleted")]
    AlreadyDeleted,
    #[error(transparent)]
    AggregateError(E),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
#[serde(tag = "lifecycle_state")]
pub enum LifecycleAggregateState<A: LifecycleAggregate> {
    NotCreated,
    Created(A),
    Deleted,
}

impl<A: LifecycleAggregate> Default for LifecycleAggregateState<A> {
    fn default() -> Self {
        Self::NotCreated
    }
}

#[async_trait]
impl<A: LifecycleAggregate> Aggregate for LifecycleAggregateState<A> {
    type Id = A::Id;
    type Command = LifecycleCommand<A>;
    type Event = LifecycleEvent<A::CreateEvent, A::UpdateEvent>;
    type Error = LifecycleError<A::Error>;
    type Services = A::Services;

    fn aggregate_type() -> String {
        A::aggregate_type()
    }

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        use LifecycleAggregateState::*;
        use LifecycleCommand::*;

        match (self, command) {
            (Deleted, _) => Err(LifecycleError::AlreadyDeleted),
            (Created(_), Create(_)) => Err(LifecycleError::AlreadyCreated),
            (NotCreated, Update(_) | Delete(_)) => Err(LifecycleError::NotCreated),
            (NotCreated, Create(create)) => {
                let (create_event, update_events) = A::handle_create(create, service)
                    .await
                    .map_err(LifecycleError::AggregateError)?;

                Ok(std::iter::once(LifecycleEvent::Created(create_event))
                    .chain(update_events.into_iter().map(LifecycleEvent::Updated))
                    .collect())
            }
            (Created(agg), Update(update)) => agg
                .handle(update, service)
                .await
                .map_err(LifecycleError::AggregateError)
                .map(|events| events.into_iter().map(LifecycleEvent::Updated).collect()),
            (Created(agg), Delete(delete)) => agg
                .handle_delete(delete, service)
                .await
                .map_err(LifecycleError::AggregateError)
                .map(|_| vec![LifecycleEvent::Deleted]),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match (self, event) {
            (this @ LifecycleAggregateState::NotCreated, LifecycleEvent::Created(create)) => {
                *this = LifecycleAggregateState::Created(A::apply_create(create))
            }
            (LifecycleAggregateState::Created(agg), LifecycleEvent::Updated(update)) => {
                agg.apply(update)
            }
            (this @ LifecycleAggregateState::Created(_), LifecycleEvent::Deleted) => {
                *this = LifecycleAggregateState::Deleted
            }
            _ => unreachable!("Invalid lifecycle for event"),
        }
    }
}
