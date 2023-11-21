use crate::api_result::ApiResult;
use crate::domain::group::{CreateGroup, GroupError, GroupId, GroupView, UpdateGroup};
use crate::domain::ticket::{TicketDestination, TicketListingViewExpandedItem};
use crate::error::{Error, GroupSnafu, PersistenceSnafu};
use crate::extractors::{Json, Path, UserContext};
use crate::related_data::{WithGroupsAndUsers, WithUsers};
use crate::routes::ticket;
use crate::state::ApplicationState;
use crate::view_repositry_ext::LifecycleViewRepositoryExt;
use axum::extract::State;
use cqrs_es::lifecycle::{LifecycleCommand, LifecycleError, LifecycleViewState};
use cqrs_es::persist::ViewRepository;
use cqrs_es::AggregateError;
use snafu::ResultExt;
use tracing::error;

pub async fn query(
    State(state): State<ApplicationState>,
    Path(id): Path<GroupId>,
) -> ApiResult<WithUsers<GroupView>> {
    ApiResult::from_async_fn(|| async {
        let view = state
            .cqrs
            .group_view_repository
            .load_lifecycle(id)
            .await
            .context(PersistenceSnafu)?
            .ok_or(Error::NotFound)?;

        WithUsers::new(state.cqrs.user_view_repository.as_ref(), view).await
    })
    .await
}

pub async fn create_command(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<GroupId>,
    Json(command): Json<CreateGroup>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .cqrs
            .group_cqrs
            .execute(
                id,
                LifecycleCommand::Create(user_context.authenticated(command)),
            )
            .await
            .context(GroupSnafu),
    )
}

pub async fn update_command(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<GroupId>,
    Json(command): Json<UpdateGroup>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .cqrs
            .group_cqrs
            .execute(
                id,
                LifecycleCommand::Update(user_context.authenticated(command)),
            )
            .await
            .context(GroupSnafu),
    )
}

pub async fn tickets_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
    Path(id): Path<GroupId>,
) -> ApiResult<WithGroupsAndUsers<Vec<TicketListingViewExpandedItem>>> {
    ApiResult::from_async_fn(|| async {
        let group_view = state
            .cqrs
            .group_view_repository
            .load(&id.0.to_string())
            .await
            .context(PersistenceSnafu)?
            .and_then(LifecycleViewState::into_created);
        let group_view = group_view.ok_or(Error::NotFound)?;
        if !group_view.members.contains(&user_context.user_id()) {
            error!(
                "User {:?} is not a member of group {:?}",
                user_context.user_id(),
                id
            );
            return Err(AggregateError::UserError(LifecycleError::AggregateError(
                GroupError::Forbidden,
            )))
            .context(GroupSnafu);
        }

        let destination_id = TicketDestination::Group(id);
        let listing = state
            .cqrs
            .ticket_destination_listing_view_repository
            .load(&destination_id.to_string())
            .await
            .context(PersistenceSnafu)?
            .unwrap_or_default();

        ticket::expand_ticket_listing_view(state, listing).await
    })
    .await
}
