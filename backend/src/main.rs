mod api_result;
mod auth;
mod config;
mod domain;
mod error;
mod extractors;
mod init_tracing;
mod login;
mod memory_view_repository;
mod state;
mod view_repositry_ext;

use crate::api_result::ApiResult;
use crate::domain::group::{GroupCommand, GroupError, GroupId, GroupView, GroupViewContent};
use crate::domain::ticket::{
    TicketCommand, TicketDestination, TicketListingView, TicketListingViewExpandedItem, TicketView,
    TicketViewContent,
};
use crate::domain::user::{IdentityView, UserCommand, UserId, UserProfileView, UserView};
use crate::error::{Error, GroupSnafu, PersistenceSnafu, TicketSnafu, UserSnafu};
use crate::extractors::{Json, Path, State, UserContext};
use crate::state::{new_application_state, ApplicationState};
use axum::routing::post;
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use cqrs_es::persist::ViewRepository;
use cqrs_es::AggregateError;
use cqrs_es::Id;
use snafu::{ResultExt, Whatever};
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{error, info, warn};

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
    router = router
        .route("/tickets/:id", get(tickets_query).post(tickets_command))
        .route("/tickets/assigned", get(ticket_assignee_listing_query))
        .route("/tickets/owned", get(ticket_owner_listing_query));

    router = router
        .route("/groups/:id", get(group_query).post(group_command))
        .route("/groups/:id/tickets", get(group_tickets_query));

    router = router
        .route("/users/me", get(me_query))
        .route("/users/:id/profile", get(user_profile_query))
        .route("/users/:id/groups", get(user_groups_query));

    router = router.route("/login/telegram", post(login::telegram_login));

    if config.expose_internal {
        warn!("Running with internal routes exposed. DO NOT USE IN PRODUCTION!");

        router = router
            .route("/users/:id", get(user_query).post(user_command))
            .route("/user-identities/:id", get(user_identity))
            .route("/fake-login/:id", post(login::fake_login))
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
) -> ApiResult<TicketViewContent> {
    ApiResult::from_async_fn(|| async {
        let ticket_view = state
            .ticket_view_repository
            .load(&id.to_string())
            .await
            .context(PersistenceSnafu)?
            .map(TicketView::unwrap);
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
            .execute(id, user_context.authenticated(command))
            .await
            .context(TicketSnafu),
    )
}

async fn expand_ticket_listing_view(
    state: ApplicationState,
    ticket_view: TicketListingView,
) -> Result<Vec<TicketListingViewExpandedItem>, Error> {
    let results = futures_util::future::join_all(
        ticket_view
            .items
            .iter()
            .map(|id| async { state.ticket_view_repository.load(&id.0.to_string()).await }),
    )
    .await;
    let results = results
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .context(PersistenceSnafu)?;
    Ok(results
        .into_iter()
        .map(|view| {
            let view = view.unwrap().unwrap();
            TicketListingViewExpandedItem {
                id: view.id,
                destination: view.destination,
                owner: view.owner,
                assignee: view.assignee,
                title: view.title,
                status: view.status,
            }
        })
        .collect())
}

async fn ticket_assignee_listing_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
) -> ApiResult<Vec<TicketListingViewExpandedItem>> {
    ApiResult::from_async_fn(|| async {
        let view = state
            .ticket_assignee_listing_view_repository
            .load(&user_context.user_id().0.to_string())
            .await
            .context(PersistenceSnafu)?
            .unwrap_or_default();

        expand_ticket_listing_view(state, view).await
    })
    .await
}

async fn ticket_owner_listing_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
) -> ApiResult<Vec<TicketListingViewExpandedItem>> {
    ApiResult::from_async_fn(|| async {
        let view = state
            .ticket_owner_listing_view_repository
            .load(&user_context.user_id().0.to_string())
            .await
            .context(PersistenceSnafu)?
            .unwrap_or_default();

        expand_ticket_listing_view(state, view).await
    })
    .await
}

async fn group_query(
    State(state): State<ApplicationState>,
    Path(id): Path<Id>,
) -> ApiResult<GroupViewContent> {
    ApiResult::from_async_fn(|| async {
        let group_view = state
            .group_view_repository
            .load(&id.to_string())
            .await
            .context(PersistenceSnafu)?
            .map(GroupView::unwrap);
        group_view.ok_or(Error::NotFound)
    })
    .await
}

async fn group_command(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<Id>,
    Json(command): Json<GroupCommand>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .group_cqrs
            .execute(id, user_context.authenticated(command))
            .await
            .context(GroupSnafu),
    )
}

async fn group_tickets_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<GroupId>,
) -> ApiResult<Vec<TicketListingViewExpandedItem>> {
    ApiResult::from_async_fn(|| async {
        let group_view = state
            .group_view_repository
            .load(&id.0.to_string())
            .await
            .context(PersistenceSnafu)?
            .map(GroupView::unwrap);
        let group_view = group_view.ok_or(Error::NotFound)?;
        if !group_view.members.contains(&user_context.user_id()) {
            error!(
                "User {:?} is not a member of group {:?}",
                user_context.user_id(),
                id
            );
            return Err(AggregateError::UserError(GroupError::Forbidden)).context(GroupSnafu);
        }

        let destination_id = TicketDestination::Group(id);
        let listing = state
            .ticket_destination_listing_view_repository
            .load(&destination_id.to_string())
            .await
            .context(PersistenceSnafu)?
            .unwrap_or_default();

        expand_ticket_listing_view(state, listing).await
    })
    .await
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
    Path(id): Path<UserId>,
) -> ApiResult<UserProfileView> {
    ApiResult::from_async_fn(|| async {
        let user_view = state
            .user_view_repository
            .load(&id.0.to_string())
            .await
            .context(PersistenceSnafu)?
            .map(UserView::profile);
        user_view.ok_or(Error::NotFound)
    })
    .await
}

async fn user_groups_query(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
) -> ApiResult<Vec<GroupViewContent>> {
    ApiResult::from_async_fn(|| async {
        let groups_view = state
            .user_groups_view_repository
            .load(&id.0.to_string())
            .await
            .context(PersistenceSnafu)?
            .unwrap_or_default();

        let results = futures_util::future::join_all(
            groups_view
                .items
                .iter()
                .map(|id| async { state.group_view_repository.load(&id.0.to_string()).await }),
        )
        .await;
        let results = results
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .context(PersistenceSnafu)?;

        Ok(results
            .into_iter()
            .map(|view| view.unwrap().unwrap())
            .collect())
    })
    .await
}

async fn user_query(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
) -> ApiResult<UserView> {
    ApiResult::from_async_fn(|| async {
        let user_view = state
            .user_view_repository
            .load(&id.0.to_string())
            .await
            .context(PersistenceSnafu)?;
        user_view.ok_or(Error::NotFound)
    })
    .await
}

async fn user_command(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
    Json(command): Json<UserCommand>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .user_cqrs
            .execute(id, command)
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
