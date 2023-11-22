use crate::api_result::ApiResult;
use crate::domain::upload::{UploadCommand, UploadError, UploadId, UploadView};
use crate::error::{Error, PersistenceSnafu};
use crate::extractors::{Json, Path, UserContext};
use crate::services::upload::UploadMetadata;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::response::Redirect;
use chrono::{DateTime, Utc};
use cqrs_es::persist::ViewRepository;
use cqrs_es::{AggregateError, Id};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::collections::BTreeMap;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InitiatedUpload {
    pub id: UploadId,
    pub url: String,
    pub fields: BTreeMap<String, String>,
    #[ts(type = "string")]
    pub expiration: DateTime<Utc>,
}

fn bad_state(view: &UploadView) -> Error {
    Error::Upload {
        source: AggregateError::UserError(match view {
            UploadView::NotInitiated => UploadError::NotInitiated,
            UploadView::Finalized { .. } => UploadError::AlreadyFinalized,
            UploadView::Dropped => UploadError::AlreadyDropped,
            UploadView::Initiated { .. } => UploadError::AlreadyInitiated,
        }),
    }
}

// TODO: I think this will just redirect to the file in the object storage
pub async fn get_file(
    State(state): State<ApplicationState>,
    // use the default, non-json extractor here
    // because this URL is not supposed to return JSON
    axum::extract::Path(id): axum::extract::Path<UploadId>,
) -> Redirect {
    let id_str = id.0.to_string();
    let view = state
        .cqrs
        .upload_view_repository
        .load(&id_str)
        .await
        .context(PersistenceSnafu)
        // error handing is aaaaa
        .unwrap()
        .ok_or(Error::NotFound)
        .unwrap();

    let UploadView::Finalized { owner, metadata } = view else {
        return Err(bad_state(&view)).unwrap();
    };

    let url = state
        .upload_service
        .make_signed_retrieve_url(owner, id, &metadata)
        .await
        .map_err(AggregateError::UserError)
        .unwrap();

    Redirect::to(&url)
}

pub async fn initiate(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Json(meta): Json<UploadMetadata>,
) -> ApiResult<InitiatedUpload> {
    ApiResult::from_async_fn(|| async move {
        let upload_id = UploadId(Id::generate());

        state
            .cqrs
            .upload_cqrs
            .execute(
                upload_id,
                user_context.authenticated(UploadCommand::Initiate(meta.clone())),
            )
            .await?;

        let presigned = state
            .upload_service
            .make_signed_upload_request(user_context.user_id(), upload_id, meta)
            .await
            .map_err(AggregateError::UserError)?;

        let upload = InitiatedUpload {
            id: upload_id,
            url: presigned.url,
            fields: presigned.fields.into_iter().collect(),
            expiration: DateTime::from_timestamp(presigned.expiration.unix_timestamp(), 0).unwrap(),
        };

        Ok(upload)
    })
    .await
}

pub async fn finalize(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<UploadId>,
) -> ApiResult<()> {
    ApiResult::from_async_fn(|| async move {
        let id_str = id.0.to_string();
        let view = state
            .cqrs
            .upload_view_repository
            .load(&id_str)
            .await
            .context(PersistenceSnafu)?
            .ok_or(Error::NotFound)?;

        let UploadView::Initiated { owner, metadata } = view else {
            return Err(bad_state(&view));
        };
        if owner != user_context.user_id() {
            return Err(Error::Upload {
                source: AggregateError::UserError(UploadError::Forbidden),
            });
        }

        state
            .upload_service
            .check_upload(owner, id, &metadata)
            .await
            .map_err(AggregateError::UserError)?;

        state
            .cqrs
            .upload_cqrs
            .execute(id, user_context.authenticated(UploadCommand::Finalize))
            .await?;

        Ok(())
    })
    .await
}
