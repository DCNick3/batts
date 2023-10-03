use crate::api_result::ApiResult;
use crate::error::{JsonSnafu, PathSnafu};
use async_trait::async_trait;
use axum::body::HttpBody;
use axum::extract::{FromRequest, FromRequestParts};
use axum::http::request::Parts;
use axum::http::Request;
use axum::BoxError;
use serde::de::DeserializeOwned;
use snafu::IntoError;

pub use axum::extract::State;

/// See [`axum::Json`] for more details.
pub struct Json<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Json<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = ApiResult;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Json::from_request(req, state).await {
            Ok(json) => Ok(Json(json.0)),
            Err(err) => Err(ApiResult::from_result(Err(JsonSnafu.into_error(err)))),
        }
    }
}

/// See [`axum::extract::Path`] for more details.
pub struct Path<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiResult;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, _state).await {
            Ok(path) => Ok(Path(path.0)),
            Err(err) => Err(ApiResult::from_result(Err(PathSnafu.into_error(err)))),
        }
    }
}
