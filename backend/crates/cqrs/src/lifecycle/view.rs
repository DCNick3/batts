use crate::lifecycle::{LifecycleAggregate, LifecycleAggregateState, LifecycleEvent};
use crate::persist::{ViewContext, ViewRepository};
use crate::{AnyId, DomainEvent, EventEnvelope, Query, View};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

type AggregateId<A> = <A as LifecycleAggregate>::Id;
type AggregateCreateEvent<A> = <A as LifecycleAggregate>::CreateEvent;
type AggregateUpdateEvent<A> = <A as LifecycleAggregate>::UpdateEvent;
pub type CreateEnvelope<'a, A> =
    LifecycleViewEventEnvelope<'a, AggregateId<A>, AggregateCreateEvent<A>>;
pub type UpdateEnvelope<'a, A> =
    LifecycleViewEventEnvelope<'a, AggregateId<A>, AggregateUpdateEvent<A>>;
type LifecycleEnvelope<A> =
    EventEnvelope<AggregateId<A>, LifecycleEvent<AggregateCreateEvent<A>, AggregateUpdateEvent<A>>>;

pub struct LifecycleViewEventEnvelope<'a, Id: AnyId, Event: DomainEvent> {
    /// The id of the aggregate instance.
    pub aggregate_id: Id,
    /// The sequence number for an aggregate instance.
    pub sequence: usize,
    /// The event payload with all business information.
    pub payload: &'a Event,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    pub metadata: &'a HashMap<String, String>,
}

pub trait LifecycleView: Debug + Serialize + DeserializeOwned + Send + Sync {
    type Aggregate: LifecycleAggregate;

    fn create(event: CreateEnvelope<'_, Self::Aggregate>) -> Self;
    fn update(&mut self, event: UpdateEnvelope<'_, Self::Aggregate>);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
pub enum LifecycleViewState<V: LifecycleView> {
    NotCreated,
    Created(V),
    Deleted,
}

impl<V: LifecycleView> LifecycleViewState<V> {
    pub fn into_created(self) -> Option<V> {
        match self {
            Self::Created(view) => Some(view),
            _ => None,
        }
    }
}

impl<V: LifecycleView> Default for LifecycleViewState<V> {
    fn default() -> Self {
        Self::NotCreated
    }
}

impl<V: LifecycleView> View for LifecycleViewState<V> {
    type Aggregate = LifecycleAggregateState<V::Aggregate>;
}

pub struct LifecycleQuery<R, V> {
    view_repository: Arc<R>,
    phantom: PhantomData<V>,
}

impl<R, V> LifecycleQuery<R, V> {
    pub fn new(view_repository: Arc<R>) -> Self {
        Self {
            view_repository,
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<R, V> Query<LifecycleAggregateState<V::Aggregate>> for LifecycleQuery<R, V>
where
    V: LifecycleView,
    R: ViewRepository<LifecycleViewState<V>>,
{
    async fn dispatch(
        &self,
        aggregate_id: AggregateId<V::Aggregate>,
        events: &[LifecycleEnvelope<V::Aggregate>],
    ) {
        let aggregate_id_str = aggregate_id.id().to_string();

        let (mut state, context) = self
            .view_repository
            .load_with_context(&aggregate_id_str)
            .await
            .expect("Persistence Error")
            .unwrap_or_else(|| {
                (
                    LifecycleViewState::default(),
                    ViewContext::new(aggregate_id_str, 0),
                )
            });

        for event in events {
            assert_eq!(aggregate_id, event.aggregate_id);

            match (&mut state, event) {
                (
                    state @ LifecycleViewState::NotCreated,
                    LifecycleEnvelope::<V::Aggregate> {
                        payload: LifecycleEvent::Created(created),
                        ..
                    },
                ) => {
                    *state = LifecycleViewState::Created(V::create(LifecycleViewEventEnvelope {
                        sequence: event.sequence,
                        aggregate_id: event.aggregate_id,
                        payload: created,
                        metadata: &event.metadata,
                    }));
                }
                (
                    LifecycleViewState::Created(state),
                    LifecycleEnvelope::<V::Aggregate> {
                        payload: LifecycleEvent::Updated(updated),
                        ..
                    },
                ) => {
                    state.update(LifecycleViewEventEnvelope {
                        sequence: event.sequence,
                        aggregate_id: event.aggregate_id,
                        payload: updated,
                        metadata: &event.metadata,
                    });
                }
                (
                    state @ LifecycleViewState::Created(_),
                    LifecycleEnvelope::<V::Aggregate> {
                        payload: LifecycleEvent::Deleted,
                        ..
                    },
                ) => {
                    *state = LifecycleViewState::Deleted;
                }
                _ => unreachable!("Invalid lifecycle event"),
            }
        }

        self.view_repository
            .update_view(state, context)
            .await
            .expect("Persistence Error");
    }
}
