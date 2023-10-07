use crate::error::ApiError as _;
use axum::body::BoxBody;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use opentelemetry::trace::TraceContextExt;
use serde::{Deserialize, Serialize};
use snafu::ErrorCompat;
use std::any::Any;
use tower_http::catch_panic::ResponseForPanic;
use tracing::{error, Span};
use tracing_opentelemetry_instrumentation_sdk as otel;

#[derive(Debug)]
pub struct ApiResult<T = ()>(pub Result<T, crate::error::Error>);

impl<T> ApiResult<T> {
    pub fn ok(value: T) -> Self {
        Self(Ok(value))
    }
    pub fn err(error: impl Into<crate::error::Error>) -> Self {
        Self(Err(error.into()))
    }
    pub fn from_result(result: Result<T, crate::error::Error>) -> Self {
        Self(result)
    }
    pub fn from_fn(f: impl FnOnce() -> Result<T, crate::error::Error>) -> Self {
        Self::from_result(f())
    }
    pub async fn from_future(
        f: impl std::future::Future<Output = Result<T, crate::error::Error>>,
    ) -> Self {
        Self::from_result(f.await)
    }
    pub async fn from_async_fn<Fn, Fut>(f: Fn) -> Self
    where
        Fn: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, crate::error::Error>>,
    {
        Self::from_result(f().await)
    }
}

impl<T: Serialize> IntoResponse for ApiResult<T> {
    fn into_response(self) -> Response {
        let (status, envelope) = match self.0 {
            Ok(result) => (StatusCode::OK, ApiEnvelope::Success(result)),
            Err(error) => (
                error.status_code(),
                ApiEnvelope::Error({
                    if error.status_code().is_server_error() {
                        error!(?error, "Internal Server Error!");
                    }

                    let context = otel::find_current_context();
                    let span = context.span();
                    let context = span.span_context();

                    let underlying_error = error.iter_chain().last().unwrap().to_string();
                    let report = snafu::Report::from_error(&error).to_string();

                    // NOTE: this is brittle, we won't __necessarily__ get the span of the HTTP request
                    let span = Span::current();
                    span.record("exception.message", &report);

                    ApiError {
                        underlying_error,
                        report,
                        trace_id: context.trace_id().to_string(),
                        span_id: context.span_id().to_string(),
                    }
                }),
            ),
        };

        (status, Json(envelope)).into_response()
    }
}

#[derive(Clone)]
pub struct PanicHandler;

impl ResponseForPanic for PanicHandler {
    type ResponseBody = BoxBody;

    fn response_for_panic(&mut self, err: Box<dyn Any + Send + 'static>) -> Response {
        let mut panic_message = None;

        if let Some(s) = err.downcast_ref::<String>() {
            error!("Service panicked: {}", s);
            panic_message = Some(s.as_str());
        } else if let Some(s) = err.downcast_ref::<&str>() {
            error!("Service panicked: {}", s);
            panic_message = Some(s);
        } else {
            error!("Service panicked but `CatchPanic` was unable to downcast the panic info");
        };

        let context = otel::find_current_context();
        let span = context.span();
        let context = span.span_context();

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiEnvelope::<()>::Error(ApiError {
                underlying_error: "The request has triggered a panic".to_string(),
                // TODO: do we want to include the panic message here?
                // probably yes, but remove it later
                report: panic_message
                    .unwrap_or("(Could not get panic message)")
                    .to_string(),
                trace_id: context.trace_id().to_string(),
                span_id: context.span_id().to_string(),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", content = "payload")]
enum ApiEnvelope<T> {
    Success(T),
    Error(ApiError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiError {
    underlying_error: String,
    report: String,
    trace_id: String,
    span_id: String,
}
