mod api_result;
mod auth;
mod config;
mod domain;
mod error;
mod extractors;
mod id;
mod init_tracing;
mod memory_view_repository;
mod state;

use crate::api_result::ApiResult;
use crate::auth::UserClaims;
use crate::domain::ticket::{TicketCommand, TicketView};
use crate::domain::user::{IdentityView, UserCommand, UserProfileView, UserView};
use crate::error::{Error, PersistenceSnafu, TicketSnafu, UserSnafu, WhateverSnafu};
use crate::extractors::{Json, Path, State, UserContext};
use crate::id::Id;
use crate::state::{new_application_state, ApplicationState};
use axum::routing::post;
use axum::{routing::get, Router};
use axum_extra::extract::CookieJar;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use cqrs_es::persist::ViewRepository;
use snafu::{ResultExt, Whatever};
use std::cell::Cell;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{info, warn};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Whatever> {
    // initialize tracing
    init_tracing::init_tracing().whatever_context("failed to initialize tracing")?;

    let environment = std::env::var("ENVIRONMENT").whatever_context(
        "Please set ENVIRONMENT env var (probably you want to use either 'prod' or 'dev')",
    )?;

    let config =
        config::Config::load(&environment).whatever_context("Loading config has failed")?;

    info!("Resolved config: {:#?}", config);

    // build our application with a route
    let app = app(&config).await;

    info!("listening on {}", config.server.endpoint);
    axum::Server::bind(&config.server.endpoint)
        .serve(app.into_make_service())
        .await
        .whatever_context("failed to run server")?;

    Ok(())
}

fn make_api_router(config: &config::Routes) -> Router<ApplicationState> {
    let mut router = Router::new();
    router = router.route("/tickets/:id", get(tickets_query).post(tickets_command));

    router = router
        .route("/users/me", get(me_query))
        .route("/users/:id/profile", get(user_profile_query));

    if config.expose_internal {
        warn!("Running with internal routes exposed. DO NOT USE IN PRODUCTION!");

        router = router
            .route("/users/:id", get(user_query).post(user_command))
            .route("/user-identities/:id", get(user_identity))
            .route("/fake-login/:id", post(fake_login))
    }

    router.fallback(fallback)
}

async fn app(config: &config::Config) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api", make_api_router(&config.routes))
        .layer(CatchPanicLayer::custom(api_result::PanicHandler))
        // include trace context as header into the response
        .layer(OtelInResponseLayer)
        // start OpenTelemetry trace on incoming request
        .layer(OtelAxumLayer::default())
        // .layer(axum::middleware::from_fn(envelope_middleware))
        .with_state(new_application_state(config).await)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn fallback() -> ApiResult<()> {
    ApiResult::from_result(Err(Error::RouteNotFound))
}

async fn tickets_query(
    State(state): State<ApplicationState>,
    Path(id): Path<Id>,
) -> ApiResult<TicketView> {
    ApiResult::from_async_fn(|| async {
        let ticket_view = state
            .ticket_view_repository
            .load(&id.to_string())
            .await
            .context(PersistenceSnafu)?;
        ticket_view.ok_or(Error::NotFound)
    })
    .await
}

async fn tickets_command(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<Id>,
    Json(command): Json<TicketCommand>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .ticket_cqrs
            .execute(&id.to_string(), user_context.authenticated(command))
            .await
            .context(TicketSnafu),
    )
}

async fn me_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
) -> ApiResult<UserView> {
    ApiResult::from_async_fn(|| async {
        let user_view = state
            .user_view_repository
            .load(&user_context.user_id().0.to_string())
            .await
            .context(PersistenceSnafu)?;
        user_view.ok_or(Error::NotFound)
    })
    .await
}

async fn user_profile_query(
    State(state): State<ApplicationState>,
    Path(id): Path<Id>,
) -> ApiResult<UserProfileView> {
    ApiResult::from_async_fn(|| async {
        let user_view = state
            .user_view_repository
            .load(&id.to_string())
            .await
            .context(PersistenceSnafu)?
            .map(UserView::profile);
        user_view.ok_or(Error::NotFound)
    })
    .await
}

async fn user_query(
    State(state): State<ApplicationState>,
    Path(id): Path<Id>,
) -> ApiResult<UserView> {
    ApiResult::from_async_fn(|| async {
        let user_view = state
            .user_view_repository
            .load(&id.to_string())
            .await
            .context(PersistenceSnafu)?;
        user_view.ok_or(Error::NotFound)
    })
    .await
}

async fn user_command(
    State(state): State<ApplicationState>,
    Path(id): Path<Id>,
    Json(command): Json<UserCommand>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .user_cqrs
            .execute(&id.to_string(), command)
            .await
            .context(UserSnafu),
    )
}

async fn user_identity(
    State(state): State<ApplicationState>,
    Path(id): Path<String>,
) -> ApiResult<IdentityView> {
    ApiResult::from_async_fn(|| async {
        let identity_view = state
            .user_identity_view_repository
            .load(&id)
            .await
            .context(PersistenceSnafu)?;
        identity_view.ok_or(Error::NotFound)
    })
    .await
}

async fn fake_login(
    jar: CookieJar,
    State(state): State<ApplicationState>,
    Path(id): Path<Id>,
) -> (CookieJar, ApiResult<()>) {
    let mut jar = Cell::new(jar);

    let result = ApiResult::from_async_fn(|| async {
        let Some(user) = state
            .user_view_repository
            .load(&id.to_string())
            .await
            .context(PersistenceSnafu)?
        else {
            return Err(Error::NotFound);
        };

        let cookie = state
            .authority
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
