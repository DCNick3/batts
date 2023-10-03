use crate::domain::ticket::TicketError;
use axum::http::StatusCode;
use cqrs_es::AggregateError;
use snafu::Snafu;

pub trait ApiError {
    fn status_code(&self) -> StatusCode;
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
    /// Persistence error
    Persistence {
        source: cqrs_es::persist::PersistenceError,
    },
    /// Error while manipulating a ticket
    Ticket { source: AggregateError<TicketError> },
    /// The requested object was not found
    NotFound,
}

impl ApiError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Json { source } => source.status(),
            Error::Path { source } => source.status(),
            Error::Persistence { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Ticket { source } => source.status_code(),
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl<T: snafu::Error + ApiError> ApiError for AggregateError<T> {
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
