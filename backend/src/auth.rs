use crate::domain::user::UserId;
use crate::error::ApiError;
use axum::http::StatusCode;
use axum_extra::extract::cookie::{Cookie, Expiration};
use chrono::Duration;
use ed25519_dalek::Keypair;
use jwt_compact::alg::Ed25519;
use jwt_compact::{AlgorithmExt, TimeOptions, Token, UntrustedToken};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu, Whatever};
use std::sync::Arc;
use time::OffsetDateTime;

#[derive(Snafu, Debug)]
pub enum AuthError {
    /// Auth cookie is missing
    NoCookie,
    /// Could not parse session token
    UnparsableToken { source: jwt_compact::ParseError },
    /// Your session token does not pass validation, probably you should relogin
    InvalidToken {
        source: jwt_compact::ValidationError,
    },
}

impl ApiError for AuthError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Clone)]
pub struct Authority {
    pub cookie_name: &'static str,
    key_pair: Arc<Keypair>,
    header: jwt_compact::Header,
    time_options: TimeOptions,
    duration: Duration,
}

impl Authority {
    pub fn new(cookie_name: &'static str, key_pair: Keypair) -> Self {
        Self {
            cookie_name,
            key_pair: Arc::new(key_pair),
            header: jwt_compact::Header::default(),
            time_options: TimeOptions::default(),
            duration: Duration::hours(3),
        }
    }

    /// Produce cookie for user authentication
    pub fn create_signed_cookie(&self, claims: UserClaims) -> Result<Cookie<'static>, Whatever> {
        let claims = jwt_compact::Claims::new(claims)
            .set_duration_and_issuance(&self.time_options, self.duration);
        let compact_token = Ed25519
            .token(self.header.clone(), &claims, &self.key_pair)
            .whatever_context("Could not create the token")?;
        Ok(Cookie::build(self.cookie_name, compact_token)
            // TODO: set secure to true when in production
            // we want to test over http, but in production we want to use https
            .secure(false)
            .http_only(false)
            .path("/")
            .expires(Expiration::DateTime(
                OffsetDateTime::from_unix_timestamp(claims.expiration.unwrap().timestamp())
                    .unwrap(),
            ))
            .finish())
    }

    pub fn extract_from_cookie(
        &self,
        cookie: Option<&Cookie>,
    ) -> Result<Token<UserClaims>, AuthError> {
        let cookie = cookie.ok_or(AuthError::NoCookie)?;
        let untrusted_token = UntrustedToken::new(cookie.value()).context(UnparsableTokenSnafu)?;

        let token = Ed25519
            .validate_integrity(&untrusted_token, &self.key_pair.public)
            .context(InvalidTokenSnafu)?;

        Ok(token)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub user_id: UserId,
    pub name: String,
}

#[derive(Debug)]
pub struct Authenticated<T> {
    pub user_id: UserId,
    pub payload: T,
}
