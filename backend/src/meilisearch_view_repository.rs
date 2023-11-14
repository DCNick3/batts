use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::View;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use tracing::{info_span, instrument, Instrument};

pub struct MeilisearchViewRepository<V> {
    index: meilisearch_sdk::Index,
    phantom: PhantomData<V>,
}

impl<V: Debug + Serialize + DeserializeOwned> MeilisearchViewRepository<V> {
    pub fn new(meilisearch: meilisearch_sdk::Client, index: &str) -> Self {
        Self {
            index: meilisearch_sdk::Index::new(index, meilisearch),
            phantom: PhantomData,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct WithId<T> {
    _view_id: String,
    #[serde(flatten)]
    data: T,
}

impl<T> WithId<T> {
    fn data(self) -> T {
        self.data
    }
}

fn map_error(error: meilisearch_sdk::Error) -> PersistenceError {
    PersistenceError::UnknownError(Box::new(error))
}

#[async_trait]
impl<V: View + Serialize + DeserializeOwned> ViewRepository<V> for MeilisearchViewRepository<V> {
    #[instrument(skip(self), fields(index = self.index.uid), err, ret)]
    async fn load(&self, view_id: &str) -> Result<Option<V>, PersistenceError> {
        self.index
            .get_document(view_id)
            .await
            .map(WithId::data)
            .map(Some)
            .or_else(|e| {
                if let meilisearch_sdk::Error::Meilisearch(meilisearch_sdk::MeilisearchError {
                    error_code: meilisearch_sdk::ErrorCode::DocumentNotFound,
                    ..
                }) = e
                {
                    Ok(None)
                } else {
                    Err(e)
                }
            })
            .map_err(map_error)
    }

    #[instrument(skip(self), fields(index = self.index.uid), err, ret)]
    async fn load_with_context(
        &self,
        view_id: &str,
    ) -> Result<Option<(V, ViewContext)>, PersistenceError> {
        self.load(view_id)
            .await
            .map(|r| r.map(|v| (v, ViewContext::new(view_id.to_string()))))
    }

    #[instrument(skip(self), fields(index = self.index.uid), err, ret)]
    async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        let task = self
            .index
            .add_or_replace(
                &[WithId {
                    _view_id: context.view_instance_id,
                    data: view,
                }],
                None,
            )
            .instrument(info_span!("add_or_replace"))
            .await
            .map_err(map_error)?;
        task.wait_for_completion(&self.index.client, None, None)
            .instrument(info_span!("wait_for_completion"))
            .await
            .map_err(map_error)?;

        Ok(())
    }
}
