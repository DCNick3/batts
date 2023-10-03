use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::{Aggregate, View};
use std::collections::HashMap;
use std::marker::PhantomData;
use tokio::sync::RwLock;

pub struct MemViewRepository<V: View<A> + Clone, A: Aggregate> {
    views: RwLock<HashMap<String, (V, ViewContext)>>,
    phantom: PhantomData<A>,
}

impl<V: View<A> + Clone, A: Aggregate> MemViewRepository<V, A> {
    pub fn new() -> Self {
        Self {
            views: RwLock::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

#[async_trait]
impl<V: View<A> + Clone, A: Aggregate> ViewRepository<V, A> for MemViewRepository<V, A> {
    async fn load(&self, view_id: &str) -> Result<Option<V>, PersistenceError> {
        Ok(self
            .views
            .read()
            .await
            .get(view_id)
            .map(|(view, _)| view.clone()))
    }

    async fn load_with_context(
        &self,
        view_id: &str,
    ) -> Result<Option<(V, ViewContext)>, PersistenceError> {
        Ok(self.views.read().await.get(view_id).map(|(view, context)| {
            (
                view.clone(),
                ViewContext::new(context.view_instance_id.clone(), context.version),
            )
        }))
    }

    async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        self.views
            .write()
            .await
            .insert(context.view_instance_id.clone(), (view, context));
        Ok(())
    }
}
