use crate::domain::upload::{S3Snafu, UploadError, UploadId};
use crate::domain::user::UserId;
use camino::{Utf8Path, Utf8PathBuf};
use s3::post_policy::{PostPolicyExpiration, PresignedPost};
use s3::{Bucket, PostPolicy, PostPolicyField, PostPolicyValue};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::collections::{BTreeSet, HashMap};
use tracing::{debug, info};
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UploadMetadata {
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

#[derive(Debug, Clone)]
pub struct UploadService {
    pub bucket: Bucket,
    pub policy: UploadPolicy,
}

impl UploadService {
    pub fn validate_upload_initiation(
        &self,
        meta: &UploadMetadata,
    ) -> Result<(), Vec<PolicyViolation>> {
        let mut violations = BTreeSet::new();

        let filename = Utf8Path::new(&meta.filename);
        if filename.as_str().contains(BAD_FILENAME_CHARS) {
            violations.insert(PolicyViolation::InvalidFilename);
        }
        match filename.extension() {
            Some(extension) => {
                if !self.policy.allowed_file_extensions.contains(extension) {
                    violations.insert(PolicyViolation::FileExtensionNotAllowed);
                }
            }
            None => {
                violations.insert(PolicyViolation::InvalidFilename);
            }
        }
        if !self
            .policy
            .allowed_content_types
            .contains(&meta.content_type)
        {
            violations.insert(PolicyViolation::ContentTypeNotAllowed);
        }
        if meta.size > self.policy.max_size {
            violations.insert(PolicyViolation::FileTooLarge);
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations.into_iter().collect())
        }
    }

    pub fn make_upload_path(
        &self,
        user: UserId,
        upload: UploadId,
        meta: &UploadMetadata,
    ) -> Utf8PathBuf {
        Utf8PathBuf::from(user.0.to_string())
            .join(upload.0.to_string())
            .join(&meta.filename)
    }

    pub async fn make_signed_upload_request(
        &self,
        user: UserId,
        upload: UploadId,
        meta: UploadMetadata,
    ) -> Result<PresignedPost, UploadError> {
        let path = self.make_upload_path(user, upload, &meta);

        info!("Creating a pre-signed request to upload to {}", path);

        PostPolicy::new(PostPolicyExpiration::ExpiresIn(1800))
            .condition(
                PostPolicyField::ContentType,
                PostPolicyValue::Exact(meta.content_type.into()),
            )
            .context(S3Snafu)?
            .condition(
                PostPolicyField::Key,
                PostPolicyValue::Exact(path.into_string().into()),
            )
            .context(S3Snafu)?
            .sign(self.bucket.clone())
            .await
            .context(S3Snafu)
    }

    pub async fn check_upload(
        &self,
        user: UserId,
        upload: UploadId,
        meta: &UploadMetadata,
    ) -> Result<(), UploadError> {
        let path = self.make_upload_path(user, upload, meta);

        info!("Checking upload at {}", path);

        let (object, _) = self.bucket.head_object(&path).await.context(S3Snafu)?;

        debug!("Object: {:?}", object);

        Ok(())
    }

    pub async fn make_signed_retrieve_url(
        &self,
        user: UserId,
        upload: UploadId,
        meta: &UploadMetadata,
    ) -> Result<String, UploadError> {
        let path = self.make_upload_path(user, upload, meta);

        let custom_queries = HashMap::from([(
            "response-content-disposition".into(),
            format!("attachment; filename=\"{}\"", meta.filename),
        )]);

        self.bucket
            .presign_get(path.into_string(), 3600, Some(custom_queries))
            .await
            .context(S3Snafu)
    }
}
