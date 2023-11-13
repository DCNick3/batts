use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::View;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::Transport;
use elasticsearch::indices::{IndicesCreateParts, IndicesDeleteParts, IndicesGetParts};
use elasticsearch::{Elasticsearch, GetParts, IndexParts};
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use snafu::{ResultExt, Snafu};
use std::fmt::Debug;
use std::marker::PhantomData;
use tracing::instrument;

pub struct ElasticViewRepository<V> {
    elastic: Elasticsearch,
    index: String,
    phantom: PhantomData<V>,
}

impl<V: Debug + Serialize + DeserializeOwned> ElasticViewRepository<V> {
    pub fn new(elastic: Elasticsearch, index: &str) -> Self {
        Self {
            elastic,
            index: index.to_string(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ElasticViewRepositoryError {
    /// Optimistic locking conflict occurred while committing and aggregate.
    OptimisticLockError,
    /// Network error while communicating with Elasticsearch.
    Network { source: elasticsearch::Error },
    /// Encountered unexpected HTTP status {status} while communicating with Elasticsearch. Response body: {body}
    Http {
        status: StatusCode,
        body: serde_json::Value,
    },
    /// Deserialization error while communicating with Elasticsearch.
    Deserialization { source: elasticsearch::Error },
}

impl ElasticViewRepositoryError {
    fn into_persistence_error(self) -> PersistenceError {
        match &self {
            Self::OptimisticLockError => PersistenceError::OptimisticLockError,
            Self::Network { .. } => PersistenceError::ConnectionError(Box::new(self)),
            Self::Http { .. } => PersistenceError::UnknownError(Box::new(self)),
            Self::Deserialization { .. } => PersistenceError::DeserializationError(Box::new(self)),
        }
    }
}

// don't you love it when you have to write type manual definitions for client libraries?
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GetResponse<V> {
    _index: String,
    _id: String,
    _version: i64,
    _seq_no: i64,
    _primary_term: i64,
    found: bool,
    _source: Option<V>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum IndexResult {
    Created,
    Updated,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IndexShards {
    total: i64,
    successful: i64,
    failed: i64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IndexResponse {
    _index: String,
    _id: String,
    _version: i64,
    result: IndexResult,
    _shards: IndexShards,
    _seq_no: i64,
    _primary_term: i64,
}

async fn check_response_status(response: Response) -> Result<Response, ElasticViewRepositoryError> {
    if !response.status_code().is_success() {
        return Err(ElasticViewRepositoryError::Http {
            status: response.status_code(),
            body: response
                .json::<serde_json::Value>()
                .await
                .context(DeserializationSnafu)?,
        });
    }

    Ok(response)
}

#[instrument(skip(transport), err, ret)]
pub async fn try_delete_index(
    transport: &Transport,
    index: &str,
) -> Result<bool, ElasticViewRepositoryError> {
    let indices = elasticsearch::indices::Indices::new(transport);

    let response = indices
        .delete(IndicesDeleteParts::Index(&[index]))
        .send()
        .await
        .context(NetworkSnafu)?;

    if response.status_code() == StatusCode::NOT_FOUND {
        return Ok(false);
    }

    check_response_status(response).await?;

    Ok(true)
}

#[instrument(skip(transport), err, ret)]
pub async fn create_index(
    transport: &Transport,
    index: &str,
) -> Result<(), ElasticViewRepositoryError> {
    let indices = elasticsearch::indices::Indices::new(transport);

    let response = indices
        .create(IndicesCreateParts::Index(index))
        .body(json! {
            {
                "settings": {
                    // we only have one ES node, so we have to set replicas to 0
                    // otherwise the index will be forever yellow
                    "index": {
                        "number_of_shards": 1,
                        "number_of_replicas": 0
                    }
                }
            }
        })
        .send()
        .await
        .context(NetworkSnafu)?;

    check_response_status(response).await?;

    Ok(())
}

#[instrument(skip(transport), err, ret)]
pub async fn ensure_index_exists(
    transport: &Transport,
    index: &str,
) -> Result<bool, ElasticViewRepositoryError> {
    let indices = elasticsearch::indices::Indices::new(transport);
    let mut response = indices
        .get(IndicesGetParts::Index(&[index]))
        .send()
        .await
        .context(NetworkSnafu)?;

    if response.status_code() == StatusCode::NOT_FOUND {
        return create_index(transport, index).await.map(|_| true);
    } else {
        response = check_response_status(response).await?;
    }

    let _response = response
        .json::<serde_json::Value>()
        .await
        .context(DeserializationSnafu)?;

    Ok(false)
}

impl<V: Debug + Serialize + DeserializeOwned> ElasticViewRepository<V> {
    #[instrument(skip(self), fields(index = self.index), err, ret)]
    async fn get(
        &self,
        view_id: &str,
    ) -> Result<Option<(V, ViewContext)>, ElasticViewRepositoryError> {
        let response = self
            .elastic
            .get(GetParts::IndexId(&self.index, view_id))
            .send()
            .await
            .context(NetworkSnafu)?;

        if response.status_code() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let response = check_response_status(response).await?;

        let response = response
            .json::<GetResponse<V>>()
            .await
            .context(DeserializationSnafu)?;

        Ok(response._source.map(|source| {
            (
                source,
                ViewContext {
                    view_instance_id: response._id,
                    version: Some((response._seq_no, response._primary_term)),
                },
            )
        }))
    }

    #[instrument(skip(self), fields(index = self.index), err, ret)]
    async fn index(&self, view: V, context: ViewContext) -> Result<(), ElasticViewRepositoryError> {
        let mut index_rq = self
            .elastic
            .index(IndexParts::IndexId(&self.index, &context.view_instance_id))
            .body(view);

        if let Some((seq_no, primary_term)) = context.version {
            index_rq = index_rq.if_seq_no(seq_no).if_primary_term(primary_term);
        }

        let response = index_rq.send().await.context(NetworkSnafu)?;

        if response.status_code() == StatusCode::CONFLICT {
            return Err(ElasticViewRepositoryError::OptimisticLockError);
        }

        let _response = check_response_status(response).await?;

        Ok(())
    }
}

#[async_trait]
impl<V: View + Serialize + DeserializeOwned> ViewRepository<V> for ElasticViewRepository<V> {
    async fn load(&self, view_id: &str) -> Result<Option<V>, PersistenceError> {
        self.get(view_id)
            .await
            .map(|v| v.map(|v| v.0))
            .map_err(ElasticViewRepositoryError::into_persistence_error)
    }

    async fn load_with_context(
        &self,
        view_id: &str,
    ) -> Result<Option<(V, ViewContext)>, PersistenceError> {
        self.get(view_id)
            .await
            .map_err(ElasticViewRepositoryError::into_persistence_error)
    }

    async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        self.index(view, context)
            .await
            .map_err(ElasticViewRepositoryError::into_persistence_error)
    }
}
