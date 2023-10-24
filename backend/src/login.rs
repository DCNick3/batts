use crate::api_result::ApiResult;
use crate::auth::UserClaims;
use crate::domain::user::{
    ExternalUserIdentity, ExternalUserProfile, TelegramLoginData, UserCommand, UserId,
};
use crate::error::{ApiError, Error, LoginSnafu, PersistenceSnafu, UserSnafu, WhateverSnafu};
use crate::extractors::{Json, Path};
use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use chrono::Utc;
use cqrs_es::persist::ViewRepository;
use cqrs_es::Id;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use snafu::{ResultExt, Snafu};
use std::cell::Cell;
use std::fmt::Debug;
use tracing::{error, info};

#[derive(Snafu, Debug)]
pub enum LoginError {
    /// Login option is not available because it was not properly configured on the server
    LoginOptionNotAvailable,
    /// Failed to verify 3rd party auth data
    InvalidAuthData,
}

impl ApiError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::LoginOptionNotAvailable => StatusCode::INTERNAL_SERVER_ERROR,
            LoginError::InvalidAuthData => StatusCode::UNAUTHORIZED,
        }
    }
}

pub async fn fake_login(
    jar: CookieJar,
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
) -> (CookieJar, ApiResult<()>) {
    let mut jar = Cell::new(jar);

    let result = ApiResult::from_async_fn(|| async {
        let Some(user) = state
            .user_view_repository
            .load(&id.0.to_string())
            .await
            .context(PersistenceSnafu)?
        else {
            return Err(Error::NotFound);
        };

        let cookie = state
            .cookie_authority
            .create_signed_cookie(UserClaims {
                user_id: user.id,
                name: user.name,
            })
            .context(WhateverSnafu)?;

        let new_jar = jar.get_mut().clone().add(cookie);
        jar.set(new_jar);

        Ok(())
    })
    .await;

    (jar.into_inner(), result)
}

/// Stores the sha256 of bot token. Used to verify the login data from telegram (see https://core.telegram.org/widgets/login).
#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct TelegramSecret(#[serde(with = "hex_serde")] pub [u8; 32]);

impl Debug for TelegramSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

fn validate_telegram_login(data: &TelegramLoginData, secret: &TelegramSecret) -> bool {
    let mut verifier =
        Hmac::<Sha256>::new_from_slice(&secret.0).expect("BUG: invalid secret length");

    // auth_date
    // first_name
    // id
    // last_name
    // photo_url
    // username

    let mut check_string_parts = vec![
        format!("auth_date={}", data.auth_date.timestamp()),
        format!("first_name={}", data.profile.first_name),
        format!("id={}", data.profile.id),
        format!("last_name={}", data.profile.last_name),
    ];
    if let Some(photo_url) = &data.profile.photo_url {
        check_string_parts.push(format!("photo_url={}", photo_url));
    }
    if let Some(username) = &data.profile.username {
        check_string_parts.push(format!("username={}", username));
    }
    let check_string = check_string_parts.join("\n");

    verifier.update(check_string.as_bytes());

    let mut hmac = [0; 32];
    if let Err(e) = hex::decode_to_slice(&data.hash, &mut hmac) {
        error!(
            "Could not decode telegram login hash `{}`: {}",
            data.hash, e
        );
        return false;
    };

    if let Err(e) = verifier.verify_slice(&hmac) {
        error!("Could not verify telegram login hash: {}", e);
        return false;
    }

    let now = Utc::now();
    if now.timestamp() - data.auth_date.timestamp() > 60 {
        error!(
            "Telegram login auth_date is too old: {} vs {}",
            data.auth_date, now
        );
        return false;
    }

    true
}

#[axum::debug_handler]
pub async fn telegram_login(
    jar: CookieJar,
    State(state): State<ApplicationState>,
    Json(data): Json<TelegramLoginData>,
) -> (CookieJar, ApiResult<()>) {
    let mut jar = Cell::new(jar);

    let result = ApiResult::from_async_fn(|| async {
        let Some(secret) = &state.telegram_login_secret else {
            error!("Telegram login is not configured");
            return Err(LoginError::LoginOptionNotAvailable).context(LoginSnafu);
        };

        if !validate_telegram_login(&data, secret) {
            error!("Telegram login data is invalid");
            return Err(LoginError::InvalidAuthData).context(LoginSnafu);
        }

        let identity = ExternalUserIdentity::Telegram(data.profile.id);

        // try to find the user with this telegram id
        let user_id = match state
            .user_identity_view_repository
            .load(&identity.to_string())
            .await
            .context(PersistenceSnafu)?
        {
            Some(user_identity_view) => user_identity_view.user_id,
            None => {
                // register the user
                let user_id = UserId(Id::generate());

                info!("User not registered, creating a new one from the telegram profile: id={} profile={:?}", user_id.0, data.profile);

                state
                    .user_cqrs
                    .execute(
                        user_id,
                        UserCommand::Create {
                            profile: ExternalUserProfile::Telegram(data.profile),
                        },
                    )
                    .await
                    .context(UserSnafu)?;
                user_id
            }
        };

        let Some(user) = state
            .user_view_repository
            .load(&user_id.0.to_string())
            .await
            .context(PersistenceSnafu)?
        else {
            return Err(Error::NotFound);
        };

        let cookie = state
            .cookie_authority
            .create_signed_cookie(UserClaims {
                user_id: user.id,
                name: user.name,
            })
            .context(WhateverSnafu)?;

        info!("Logging in as {}", user_id.0);

        let new_jar = jar.get_mut().clone().add(cookie);
        jar.set(new_jar);

        Ok(())
    })
    .await;

    (jar.into_inner(), result)
}
