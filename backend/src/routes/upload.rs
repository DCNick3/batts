use crate::api_result::ApiResult;
use crate::error::{ApiError, UploadSnafu};
use crate::extractors::{Json, UserContext};
use crate::state::ApplicationState;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use camino::{Utf8Path, Utf8PathBuf};
use chrono::{DateTime, Utc};
use cqrs_es::Id;
use futures_util::FutureExt;
use s3::post_policy::{PostPolicyExpiration, PresignedPost};
use s3::{Bucket, PostPolicy, PostPolicyField, PostPolicyValue};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::collections::{BTreeMap, BTreeSet};
use tracing::info;
use ts_rs::TS;

#[derive(
    Default, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, TS, Serialize, Deserialize,
)]
#[ts(export)]
pub struct UploadId(pub Id);

#[derive(Snafu, Debug)]
pub enum UploadError {
    /// Upload option is not available because it was not properly configured on the server
    NotConfigured,
    /// Upload policy violated: `{violations:?}`
    PolicyViolated { violations: Vec<PolicyViolation> },
    /// Error in the underlying S3 library
    S3 { source: s3::error::S3Error },
}

impl ApiError for UploadError {
    fn status_code(&self) -> StatusCode {
        match self {
            UploadError::NotConfigured => StatusCode::INTERNAL_SERVER_ERROR,
            UploadError::PolicyViolated { .. } => StatusCode::BAD_REQUEST,
            UploadError::S3 { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UploadState {
    pub bucket: Bucket,
    pub policy: UploadPolicy,
}

#[async_trait]
impl FromRequestParts<ApplicationState> for UploadState {
    type Rejection = ApiResult;

    async fn from_request_parts(
        _: &mut Parts,
        state: &ApplicationState,
    ) -> Result<Self, Self::Rejection> {
        match state.upload.clone() {
            Some(s3) => Ok(s3),
            None => Err(ApiResult::from_result(
                Err(UploadError::NotConfigured).context(UploadSnafu),
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InitiateUpload {
    filename: String,
    content_type: String,
    size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UploadPolicy {
    allowed_file_extensions: BTreeSet<String>,
    allowed_content_types: BTreeSet<String>,
    max_size: u64,
}

#[derive(Snafu, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum PolicyViolation {
    /// Invalid filename
    InvalidFilename,
    /// File extension not allowed
    FileExtensionNotAllowed,
    /// Content type not allowed
    ContentTypeNotAllowed,
    /// File too large
    FileTooLarge,
}

const BAD_FILENAME_CHARS: &[char] = &['/', '\\', '?', '%', '*', ':', '|', '"', '<', '>'];

impl InitiateUpload {
    pub fn validate(&self, policy: &UploadPolicy) -> Result<(), Vec<PolicyViolation>> {
        let mut violations = BTreeSet::new();

        let filename = Utf8Path::new(&self.filename);
        if filename.as_str().contains(BAD_FILENAME_CHARS) {
            violations.insert(PolicyViolation::InvalidFilename);
        }
        match filename.extension() {
            Some(extension) => {
                if !policy.allowed_file_extensions.contains(extension) {
                    violations.insert(PolicyViolation::FileExtensionNotAllowed);
                }
            }
            None => {
                violations.insert(PolicyViolation::InvalidFilename);
            }
        }
        if !policy.allowed_content_types.contains(&self.content_type) {
            violations.insert(PolicyViolation::ContentTypeNotAllowed);
        }
        if self.size > policy.max_size {
            violations.insert(PolicyViolation::FileTooLarge);
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations.into_iter().collect())
        }
    }

    pub async fn make_signed_request(
        self,
        bucket: Bucket,
        path: &str,
    ) -> Result<PresignedPost, s3::error::S3Error> {
        PostPolicy::new(PostPolicyExpiration::ExpiresIn(1800))
            .condition(
                PostPolicyField::ContentType,
                PostPolicyValue::Exact(self.content_type.into()),
            )?
            .condition(PostPolicyField::Key, PostPolicyValue::Exact(path.into()))?
            .sign(bucket)
            .await
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InitiatedUpload {
    pub id: UploadId,
    pub url: String,
    pub fields: BTreeMap<String, String>,
    #[ts(type = "string")]
    pub expiration: DateTime<Utc>,
}

// pub struct

pub async fn initiate(
    state: UploadState,
    user_context: UserContext,
    Json(upload): Json<InitiateUpload>,
) -> ApiResult<InitiatedUpload> {
    ApiResult::from_async_fn(|| {
        async move {
            upload
                .validate(&state.policy)
                .map_err(|violations| UploadError::PolicyViolated { violations })?;

            let upload_id = UploadId(Id::generate());
            let path = Utf8PathBuf::from(user_context.user_id().0.to_string())
                .join(upload_id.0.to_string())
                .join(&upload.filename);

            let presigned = upload
                .make_signed_request(state.bucket, path.as_str())
                .await
                .context(S3Snafu)?;

            info!("Initiating an upload to {}", path);

            let upload = InitiatedUpload {
                id: upload_id,
                url: presigned.url,
                fields: presigned.fields.into_iter().collect(),
                expiration: DateTime::from_timestamp(presigned.expiration.unix_timestamp(), 0)
                    .unwrap(),
            };

            Ok(upload)
        }
        .map(|r: Result<_, UploadError>| r.context(UploadSnafu))
    })
    .await
}

// pub async fn finalize(state: UploadState, user_context: UserContext)
