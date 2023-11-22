mod group;
mod login;
mod search;
mod ticket;
mod upload;
mod user;

use crate::state::{ApplicationState, BattsAggregate, BattsView, CqrsState};
use axum::extract::State;
use axum::routing::{get, post};
use axum::Router;
use cqrs_es::lifecycle::{LifecycleAggregate, LifecycleCommand, LifecycleError, LifecycleView};
use cqrs_es::AggregateError;
use indexmap::IndexSet;
use serde::de::DeserializeOwned;
use snafu::ResultExt;
use tracing::warn;

use crate::api_result::ApiResult;
use crate::error::{Error, PersistenceSnafu};

use crate::auth::Authenticated;
use crate::domain::group::{Group, GroupId, GroupView};
use crate::domain::ticket::{Ticket, TicketView};
use crate::domain::user::UserId;
use crate::extractors::{Json, Path, UserContext};
use crate::related_data::{CollectIds, ViewWithRelated, WithGroupsAndUsers, WithUsers};
use crate::view_repositry_ext::LifecycleViewRepositoryExt as _;
pub use login::LoginError;

async fn generic_query<R: ViewWithRelated>(
    State(state): State<ApplicationState>,
    Path(id): Path<<<R::View as LifecycleView>::Aggregate as LifecycleAggregate>::Id>,
) -> ApiResult<R>
where
    <R as ViewWithRelated>::View: LifecycleView + BattsView,
    <<R as ViewWithRelated>::View as LifecycleView>::Aggregate: BattsAggregate,
{
    ApiResult::from_async_fn(|| async {
        let repository = <R::View as BattsView>::get_view_repository(&state.cqrs);

        let view = repository
            .load_lifecycle(id)
            .await
            .context(PersistenceSnafu)?
            .ok_or(Error::NotFound)?;

        R::new(&state.cqrs, view).await
    })
    .await
}

async fn verify_command<C>(state: &CqrsState, command: &C) -> Result<(), Error>
where
    C: CollectIds<UserId> + CollectIds<GroupId>,
{
    let mut user_ids = IndexSet::new();
    let mut group_ids = IndexSet::new();
    command.collect_ids(&mut user_ids);
    command.collect_ids(&mut group_ids);

    fn map_error(e: Error) -> Error {
        match e {
            Error::ViewRelatedItemNotFound => Error::CommandRelatedItemNotFound,
            e => e,
        }
    }

    super::related_data::retrieve_users(state.user_view_repository.as_ref(), user_ids)
        .await
        .map_err(map_error)?;
    super::related_data::retrieve_groups(state.group_view_repository.as_ref(), group_ids)
        .await
        .map_err(map_error)?;

    Ok(())
}

async fn generic_authenticated_create_command<A, C>(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<A::Id>,
    Json(command): Json<C>,
) -> ApiResult
where
    A: BattsAggregate<CreateCommand = Authenticated<C>>,
    C: DeserializeOwned + CollectIds<UserId> + CollectIds<GroupId> + 'static,
    AggregateError<LifecycleError<A::Error>>: Into<Error>,
{
    ApiResult::from_async_fn(move || async move {
        let cqrs = A::get_cqrs_state(&state.cqrs);
        verify_command(&state.cqrs, &command).await?;
        cqrs.execute(
            id,
            LifecycleCommand::Create(user_context.authenticated(command)),
        )
        .await
        .map_err(Into::into)
    })
    .await
}

async fn generic_authenticated_update_command<A, C>(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<A::Id>,
    Json(command): Json<C>,
) -> ApiResult
where
    A: BattsAggregate<UpdateCommand = Authenticated<C>>,
    C: DeserializeOwned + CollectIds<UserId> + CollectIds<GroupId> + 'static,
    AggregateError<LifecycleError<A::Error>>: Into<Error>,
{
    ApiResult::from_async_fn(move || async move {
        let cqrs = A::get_cqrs_state(&state.cqrs);
        verify_command(&state.cqrs, &command).await?;
        cqrs.execute(
            id,
            LifecycleCommand::Update(user_context.authenticated(command)),
        )
        .await
        .map_err(Into::into)
    })
    .await
}

pub fn make_api_router(config: &crate::config::Routes) -> Router<ApplicationState> {
    let mut router = Router::new();
    router = router
        .route(
            "/tickets/:id",
            get(generic_query::<WithGroupsAndUsers<TicketView>>)
                .put(generic_authenticated_create_command::<Ticket, _>)
                .post(generic_authenticated_update_command::<Ticket, _>),
        )
        .route("/tickets/assigned", get(ticket::assignee_listing_query))
        .route("/tickets/owned", get(ticket::owned_listing_query));

    router = router
        .route(
            "/groups/:id",
            get(generic_query::<WithUsers<GroupView>>)
                .put(generic_authenticated_create_command::<Group, _>)
                .post(generic_authenticated_update_command::<Group, _>),
        )
        .route("/groups/:id/tickets", get(group::tickets_query));

    router = router
        .route("/users/me", get(user::me_query))
        .route("/users/:id/profile", get(user::profile_query))
        .route("/users/:id/groups", get(user::groups_query));

    router = router.route("/login/telegram", post(login::telegram_login));

    router = router
        .route("/upload/:id/file", get(upload::get_file))
        .route("/upload/:id/file/:filename", get(upload::get_file))
        .route("/upload/initiate", post(upload::initiate))
        .route("/upload/:id/finalize", post(upload::finalize));

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
