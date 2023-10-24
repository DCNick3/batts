use async_trait::async_trait;
use cqrs_es::lifecycle::{LifecycleAggregate, LifecycleView, LifecycleViewState};
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::{AnyId, View};

#[async_trait]
pub trait ViewRepositoryExt<V>: ViewRepository<V>
where
    V: View,
{
    async fn load_modify_update<U, D>(
        &self,
        view_id: &str,
        update: U,
        default: D,
    ) -> Result<(), PersistenceError>
    where
        U: FnOnce(&mut V) + Send + Sync,
        D: FnOnce() -> V + Send + Sync,
    {
        let (mut view, context) = self
            .load_with_context(view_id)
            .await
            .unwrap()
            .unwrap_or_else(|| (default(), ViewContext::new(view_id.to_string(), 0)));

        update(&mut view);

        self.update_view(view, context).await?;

        Ok(())
    }

    async fn load_modify_update_default<U>(
        &self,
        view_id: &str,
        update: U,
    ) -> Result<(), PersistenceError>
    where
        U: FnOnce(&mut V) + Send + Sync,
        V: Default,
    {
        self.load_modify_update(view_id, update, || V::default())
            .await
    }
}

impl<V, T> ViewRepositoryExt<V> for T
where
    V: View,
    T: ViewRepository<V>,
{
}

#[async_trait]
pub trait LifecycleViewRepositoryExt<V>: ViewRepository<LifecycleViewState<V>>
where
    V: LifecycleView,
{
    async fn load_lifecycle(
        &self,
        id: <V::Aggregate as LifecycleAggregate>::Id,
    ) -> Result<Option<V>, PersistenceError> {
        let id_str = id.id().to_string();

        Ok(self
            .load(&id_str)
            .await
            .expect("Failed to load view")
            .and_then(|v| v.into_created()))
    }
}

impl<V, T> LifecycleViewRepositoryExt<V> for T
where
    V: LifecycleView,
    T: ViewRepository<LifecycleViewState<V>>,
{
}
