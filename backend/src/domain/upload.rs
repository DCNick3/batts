use crate::auth::Authenticated;
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::related_data::CollectIds;
use crate::services::upload::{PolicyViolation, UploadMetadata, UploadService};
use async_trait::async_trait;
use cqrs_es::persist::{ViewContext, ViewRepository};
use cqrs_es::{Aggregate, AnyId, DomainEvent, EventEnvelope, Id, Query, View};
use http::StatusCode;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::Arc;
use ts_rs::TS;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct UploadId(pub Id);

impl AnyId for UploadId {
    fn from_id(id: Id) -> Self {
        Self(id)
    }

    fn id(&self) -> Id {
        self.0
    }
}

impl CollectIds<UserId> for UploadId {
    fn collect_ids(&self, _: &mut IndexSet<UserId>) {}
}

impl CollectIds<super::group::GroupId> for UploadId {
    fn collect_ids(&self, _: &mut IndexSet<super::group::GroupId>) {}
}

pub enum UploadCommand {
    /// Start the upload process, generate a presigned URL for the client to upload the file to
    Initiate(UploadMetadata),
    /// Finalize the upload, making it available for other users. The server will verify that the upload was completed successfully
    Finalize,
    /// Cancel upload. This will be done automatically if the upload is not finalized within a certain time
    Drop,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum UploadEvent {
    Initiated {
        metadata: UploadMetadata,
        owner: UserId,
    },
    Finalized,
    Dropped,
}

impl DomainEvent for UploadEvent {
    fn event_type(&self) -> String {
        match self {
            UploadEvent::Initiated { .. } => "Initiated".to_string(),
            UploadEvent::Finalized => "Finalized".to_string(),
            UploadEvent::Dropped => "Dropped".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum UploadError {
    // State-related errors are kinda funky and do not describe all the possible states
    /// The upload was not initiated yet
    NotInitiated,
    /// The upload was already initiated
    AlreadyInitiated,
    /// The upload was already finalized
    AlreadyFinalized,
    /// The upload was already dropped
    AlreadyDropped,
    /// User cannot access this upload
    Forbidden,
    /// Upload policy violated: `{violations:?}`
    PolicyViolated { violations: Vec<PolicyViolation> },
    /// Error in the underlying S3 library
    S3 { source: s3::error::S3Error },
}

impl ApiError for UploadError {
    fn status_code(&self) -> StatusCode {
        match self {
            UploadError::NotInitiated
            | UploadError::AlreadyInitiated
            | UploadError::AlreadyFinalized
            | UploadError::AlreadyDropped
            | UploadError::PolicyViolated { .. } => StatusCode::BAD_REQUEST,
            UploadError::Forbidden => StatusCode::FORBIDDEN,
            UploadError::S3 { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum Upload {
    #[default]
    NotInitiated,
    Initiated {
        metadata: UploadMetadata,
        owner: UserId,
    },
    Finalized {
        metadata: UploadMetadata,
        owner: UserId,
    },
    Dropped,
}

#[async_trait]
impl Aggregate for Upload {
    type Id = UploadId;
    type Command = Authenticated<UploadCommand>;
    type Event = UploadEvent;
    type Error = UploadError;
    type Services = Arc<UploadService>;

    fn aggregate_type() -> String {
        "Upload".to_string()
    }

    async fn handle(
        &self,
        Authenticated {
            user_id: performer,
            payload: command,
        }: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        let mut events = Vec::new();
        match (self, command) {
            (Upload::NotInitiated, UploadCommand::Initiate(metadata)) => {
                // TODO: check user quota (don't care for now)
                service
                    .validate_upload_initiation(&metadata)
                    .map_err(|violations| UploadError::PolicyViolated { violations })?;
                // NOTE: the presigned link generation is done OUTSIDE of the command handler
                // there's some logic stacked on top in the route handler
                events.push(UploadEvent::Initiated {
                    metadata,
                    owner: performer,
                });
            }
            (Upload::Initiated { owner, .. }, UploadCommand::Finalize) => {
                if owner != &performer {
                    return Err(UploadError::Forbidden);
                }
                events.push(UploadEvent::Finalized);
            }
            (Upload::Initiated { owner, .. }, UploadCommand::Drop) => {
                if owner != &performer {
                    return Err(UploadError::Forbidden);
                }
                events.push(UploadEvent::Dropped);
            }
            // error conditions
            (Upload::NotInitiated, _) => return Err(UploadError::NotInitiated),
            (Upload::Initiated { .. }, _) => return Err(UploadError::AlreadyInitiated),
            (Upload::Finalized { .. }, _) => return Err(UploadError::AlreadyFinalized),
            (Upload::Dropped, _) => return Err(UploadError::AlreadyDropped),
        };

        Ok(events)
    }

    fn apply(&mut self, event: Self::Event) {
        match (self.clone(), event) {
            (Upload::NotInitiated, UploadEvent::Initiated { metadata, owner }) => {
                *self = Upload::Initiated { metadata, owner }
            }
            (Upload::Initiated { owner, metadata }, UploadEvent::Finalized) => {
                *self = Upload::Finalized { metadata, owner }
            }
            (Upload::Initiated { .. }, UploadEvent::Dropped) => {
                *self = Upload::Dropped;
            }
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum UploadView {
    #[default]
    NotInitiated,
    Initiated {
        metadata: UploadMetadata,
        owner: UserId,
    },
    Finalized {
        metadata: UploadMetadata,
        owner: UserId,
    },
    Dropped,
}

impl View for UploadView {
    type Aggregate = Upload;
}

pub struct UploadQuery<R> {
    view_repository: Arc<R>,
}

impl<R> UploadQuery<R> {
    pub fn new(view_repository: Arc<R>) -> Self {
        Self { view_repository }
    }
}

#[async_trait]
impl<R: ViewRepository<UploadView>> Query<Upload> for UploadQuery<R> {
    async fn dispatch(
        &self,
        aggregate_id: UploadId,
        events: &[EventEnvelope<UploadId, UploadEvent>],
    ) {
        let aggregate_id_str = aggregate_id.id().to_string();

        let (mut state, context) = self
            .view_repository
            .load_with_context(&aggregate_id_str)
            .await
            .expect("Persistence Error")
            .unwrap_or_else(|| (UploadView::default(), ViewContext::new(aggregate_id_str)));

        for event in events {
            assert_eq!(aggregate_id, event.aggregate_id);

            match (state.clone(), event) {
                (
                    UploadView::NotInitiated,
                    EventEnvelope {
                        payload: UploadEvent::Initiated { metadata, owner },
                        ..
                    },
                ) => {
                    state = UploadView::Initiated {
                        metadata: metadata.clone(),
                        owner: owner.clone(),
                    }
                }
                (
                    UploadView::Initiated { metadata, owner },
                    EventEnvelope {
                        payload: UploadEvent::Finalized,
                        ..
                    },
                ) => state = UploadView::Finalized { metadata, owner },
                (
                    UploadView::Initiated { .. },
                    EventEnvelope {
                        payload: UploadEvent::Dropped,
                        ..
                    },
                ) => {
                    state = UploadView::Dropped;
                }
                _ => unreachable!("Invalid upload event"),
            }
        }

        self.view_repository
            .update_view(state, context)
            .await
            .expect("Persistence Error");
    }
}
