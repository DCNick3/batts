use crate::lifecycle::{LifecycleAggregate, LifecycleAggregateState};
use crate::{EventEnvelope, View};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait LifecycleView: Debug + Serialize + DeserializeOwned + Send + Sync {
    type Aggregate: LifecycleAggregate;

    fn create(
        &mut self,
        event: &EventEnvelope<
            <Self::Aggregate as LifecycleAggregate>::Id,
            <Self::Aggregate as LifecycleAggregate>::CreateEvent,
        >,
    );
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = ""))]
pub enum LifecycleViewState<V: LifecycleView> {
    NotCreated,
    Created(V),
    Deleted,
}

impl<V: LifecycleView> Default for LifecycleViewState<V> {
    fn default() -> Self {
        Self::NotCreated
    }
}

impl<V: LifecycleView> View for LifecycleViewState<V> {
    type Aggregate = LifecycleAggregateState<V::Aggregate>;
}
