use crate::persist::event_stream::ReplayStream;
use crate::persist::{PersistenceError, SerializedEvent, SerializedSnapshot};
use crate::{Aggregate, Id};
use async_trait::async_trait;
use serde_json::Value;

/// Handles the database access needed for operation of a PersistedSnapshotStore.
#[async_trait]
pub trait PersistedEventRepository: Send + Sync {
    /// Returns all events for a single aggregate instance.
    async fn get_events<A: Aggregate>(
        &self,
        aggregate_id: Id,
    ) -> Result<Vec<SerializedEvent>, PersistenceError>;

    /// Returns the last events for a single aggregate instance.
    async fn get_last_events<A: Aggregate>(
        &self,
        aggregate_id: Id,
        last_sequence: usize,
    ) -> Result<Vec<SerializedEvent>, PersistenceError>;

    /// Returns the current snapshot for an aggregate instance.
    async fn get_snapshot<A: Aggregate>(
        &self,
        aggregate_id: Id,
    ) -> Result<Option<SerializedSnapshot>, PersistenceError>;

    /// Commits the updated aggregate and accompanying events.
    async fn persist<A: Aggregate>(
        &self,
        events: &[SerializedEvent],
        snapshot_update: Option<(Id, Value, usize)>,
    ) -> Result<(), PersistenceError>;

    /// Streams all events for an aggregate instance.
    async fn stream_events<A: Aggregate>(
        &self,
        aggregate_id: Id,
    ) -> Result<ReplayStream, PersistenceError>;

    /// Streams all events for an aggregate type.
    async fn stream_all_events<A: Aggregate>(&self) -> Result<ReplayStream, PersistenceError>;
}
