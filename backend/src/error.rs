use crate::domain::group::GroupError;
use crate::domain::ticket::TicketError;
use crate::domain::user::UserError;
use axum::http::StatusCode;
use cqrs_es::lifecycle::LifecycleError;
use cqrs_es::AggregateError;
use snafu::{Backtrace, Snafu};
use std::error::Error as _;

pub trait ApiError {
    fn status_code(&self) -> StatusCode;
}

#[derive(Debug, Snafu)]
#[snafu(whatever)]
#[snafu(display("{message}"))]
#[snafu(provide(opt, ref, chain, dyn std::error::Error => source.as_deref()))]
pub struct Whatever {
    #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
    #[snafu(provide(false))]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
    message: String,
    backtrace: Backtrace,
}

impl Whatever {
    /// Gets the backtrace from the deepest `Whatever` error. If none
    /// of the underlying errors are `Whatever`, returns the backtrace
    /// from when this instance was created.
    #[allow(unused)]
    pub fn backtrace(&self) -> Option<&Backtrace> {
        let mut best_backtrace = &self.backtrace;

        let mut source = self.source();
        while let Some(s) = source {
            if let Some(this) = s.downcast_ref::<Self>() {
                best_backtrace = &this.backtrace;
            }
            source = s.source();
        }

        Some(best_backtrace)
    }
}

/// An error to rule them all
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    /// Error while extracting a json body
    Json {
        source: axum::extract::rejection::JsonRejection,
    },
    /// Error while extracting value from path
    Path {
        source: axum::extract::rejection::PathRejection,
    },
    /// Error while extracting value from query string
    Query {
        source: axum::extract::rejection::QueryRejection,
    },
    /// Persistence error
    Persistence {
        source: cqrs_es::persist::PersistenceError,
    },
    /// Error while interacting with Meilisearch
    Meilisearch { source: meilisearch_sdk::Error },
    /// Auth error
    Auth { source: crate::auth::AuthError },
    /// Login error
    Login { source: crate::routes::LoginError },
    /// Upload error
    Upload { source: crate::routes::UploadError },
    /// Error while manipulating a ticket
    #[snafu(context(false))] // implement From conversion
    Ticket {
        source: AggregateError<LifecycleError<TicketError>>,
    },
    /// Error while manipulating a group
    #[snafu(context(false))]
    Group {
        source: AggregateError<LifecycleError<GroupError>>,
    },
    /// Error while manipulating a user
    #[snafu(context(false))]
    User {
        source: AggregateError<LifecycleError<UserError>>,
    },
    /// The requested object was not found
    NotFound,
    /// Inconsistency in the database: a related item was not found
    ViewRelatedItemNotFound,
    /// Bad request: a command referenced a related item that was not found
    CommandRelatedItemNotFound,
    /// Could not find a route for the request
    RouteNotFound,
    /// A valid auth cookie provided, but the authenticated user does not exist in the database
    AuthenticatedUserNotFound,
    /// Internal error
    Whatever { source: Whatever },
}

impl ApiError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Json { source } => source.status(),
            Error::Path { source } => source.status(),
            Error::Query { source } => source.status(),
            Error::Persistence { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Meilisearch { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Auth { source } => source.status_code(),
            Error::Login { source } => source.status_code(),
            Error::Upload { source } => source.status_code(),
            Error::Ticket { source } => source.status_code(),
            Error::Group { source } => source.status_code(),
            Error::User { source } => source.status_code(),
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::ViewRelatedItemNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            Error::CommandRelatedItemNotFound => StatusCode::BAD_REQUEST,
            Error::RouteNotFound => StatusCode::NOT_FOUND,
            Error::AuthenticatedUserNotFound => StatusCode::UNAUTHORIZED,
            Error::Whatever { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl<E: snafu::Error + ApiError> ApiError for AggregateError<E> {
    fn status_code(&self) -> StatusCode {
        match self {
            AggregateError::UserError(e) => e.status_code(),
            AggregateError::AggregateConflict => {
                todo!()
            }
            AggregateError::DatabaseConnectionError(_) => {
                todo!()
            }
            AggregateError::DeserializationError(_) => {
                todo!()
            }
            AggregateError::UnexpectedError(_) => {
                todo!()
            }
        }
    }
}

impl<E: snafu::Error + ApiError> ApiError for LifecycleError<E> {
    fn status_code(&self) -> StatusCode {
        match self {
            LifecycleError::NotCreated => StatusCode::NOT_FOUND,
            LifecycleError::AlreadyCreated => StatusCode::CONFLICT,
            LifecycleError::AlreadyDeleted => StatusCode::NOT_FOUND,
            LifecycleError::AggregateError(e) => e.status_code(),
        }
    }
}
