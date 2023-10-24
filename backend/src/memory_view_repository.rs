use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::View;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct MemViewRepository<V: View + Clone> {
    views: RwLock<HashMap<String, (V, ViewContext)>>,
}

impl<V: View + Clone> MemViewRepository<V> {
    pub fn new() -> Self {
        Self {
            views: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl<V: View + Clone> ViewRepository<V> for MemViewRepository<V> {
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
