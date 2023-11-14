mod group;
mod login;
mod search;
mod ticket;
mod upload;
mod user;

use crate::state::ApplicationState;
use axum::routing::{get, post};
use axum::Router;
use tracing::warn;

use crate::api_result::ApiResult;
use crate::error::Error;

pub use login::LoginError;
pub use upload::{UploadError, UploadPolicy, UploadState};

pub fn make_api_router(config: &crate::config::Routes) -> Router<ApplicationState> {
    let mut router = Router::new();
    router = router
        .route(
            "/tickets/:id",
            get(ticket::query)
                .put(ticket::create_command)
                .post(ticket::update_command),
        )
        .route("/tickets/assigned", get(ticket::assignee_listing_query))
        .route("/tickets/owned", get(ticket::owned_listing_query));

    router = router
        .route(
            "/groups/:id",
            get(group::query)
                .put(group::create_command)
                .post(group::update_command),
        )
        .route("/groups/:id/tickets", get(group::tickets_query));

    router = router
        .route("/users/me", get(user::me_query))
        .route("/users/:id/profile", get(user::profile_query))
        .route("/users/:id/groups", get(user::groups_query));

    router = router.route("/login/telegram", post(login::telegram_login));

    router = router.route("/upload/initiate", post(upload::initiate));

    router = router
        .route("/search/tickets", get(search::tickets))
        .route("/search/users", get(search::users))
        .route("/search/groups", get(search::groups));

    if config.expose_internal {
        warn!("Running with internal routes exposed. DO NOT USE IN PRODUCTION!");

        router = router
            .route(
                "/users/:id",
                get(user::internal_query)
                    .put(user::internal_create_command)
                    .post(user::internal_update_command),
            )
            .route("/user-identities/:id", get(user::internal_identity))
            .route("/fake-login/:id", post(login::internal_fake_login))
    }

    router.fallback(fallback)
}

async fn fallback() -> ApiResult<()> {
    ApiResult::from_result(Err(Error::RouteNotFound))
}
