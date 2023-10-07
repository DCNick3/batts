mod api_result;
mod domain;
mod error;
mod extractors;
mod id;
mod init_tracing;
mod memory_view_repository;
mod state;

use crate::api_result::ApiResult;
use crate::domain::ticket::{TicketCommand, TicketView};
use crate::domain::user::{IdentityView, UserCommand, UserView};
use crate::error::{Error, PersistenceSnafu, TicketSnafu, UserSnafu};
use crate::extractors::{Json, Path, State};
use crate::id::Id;
use crate::state::{new_application_state, ApplicationState};
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use cqrs_es::persist::ViewRepository;
use snafu::{ResultExt, Whatever};
use std::net::SocketAddr;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{debug, warn};

#[tokio::main]
async fn main() -> Result<(), Whatever> {
    // initialize tracing
    init_tracing::init_tracing().whatever_context("failed to initialize tracing")?;

    // build our application with a route
    let app = app().await;

    if cfg!(feature = "expose-internal-routes") {
        warn!("Running with internal routes exposed. DO NOT USE IN PRODUCTION!");
    }

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .whatever_context("failed to run server")?;

    Ok(())
}

fn make_api_router() -> Router<ApplicationState> {
    let mut router = Router::new();
    router = router.route("/tickets/:id", get(tickets_query).post(tickets_command));

    if cfg!(feature = "expose-internal-routes") {
        router = router
            .route("/users/:id", get(user_query).post(user_command))
            .route("/user-identities/:id", get(user_identity));
    }

    router.fallback(fallback)
}

async fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api", make_api_router())
        .layer(CatchPanicLayer::custom(api_result::PanicHandler))
        // include trace context as header into the response
        .layer(OtelInResponseLayer)
        // start OpenTelemetry trace on incoming request
        .layer(OtelAxumLayer::default())
        // .layer(axum::middleware::from_fn(envelope_middleware))
        .with_state(new_application_state().await)
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
    Path(id): Path<Id>,
    Json(command): Json<TicketCommand>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .ticket_cqrs
            .execute(&id.to_string(), command)
            .await
            .context(TicketSnafu),
    )
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
