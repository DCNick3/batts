use crate::api_result::ApiResult;
use crate::error::{AuthSnafu, JsonSnafu, PathSnafu, QuerySnafu};
use async_trait::async_trait;
use axum::body::HttpBody;
use axum::extract::{FromRequest, FromRequestParts};
use axum::http::request::Parts;
use axum::http::Request;
use axum::BoxError;
use serde::de::DeserializeOwned;
use snafu::{IntoError, ResultExt};

use crate::auth::{Authenticated, UserClaims};
use crate::domain::user::UserId;
use crate::state::ApplicationState;
use axum_extra::extract::CookieJar;

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

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(path) => Ok(Path(path.0)),
            Err(err) => Err(ApiResult::err(PathSnafu.into_error(err))),
        }
    }
}

/// See [`axum::extract::Query`] for more details.
pub struct Query<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiResult;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(query) => Ok(Query(query.0)),
            Err(err) => Err(ApiResult::err(QuerySnafu.into_error(err))),
        }
    }
}

pub struct UserContext(UserClaims);

impl UserContext {
    pub fn user_id(&self) -> UserId {
        self.0.user_id
    }
    pub fn authenticated<T>(&self, payload: T) -> Authenticated<T> {
        Authenticated {
            user_id: self.0.user_id,
            payload,
        }
    }
}

#[async_trait]
impl FromRequestParts<ApplicationState> for UserContext {
    type Rejection = ApiResult;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApplicationState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_headers(&parts.headers);

        let token = state
            .cookie_authority
            .extract_from_cookie(cookies.get(state.cookie_authority.cookie_name))
            .context(AuthSnafu)
            .map_err(ApiResult::err)?;

        Ok(Self(token.claims().custom.clone()))
    }
}
