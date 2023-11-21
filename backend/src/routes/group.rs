use crate::api_result::ApiResult;
use crate::domain::group::{GroupError, GroupId};
use crate::domain::ticket::{TicketDestination, TicketListingViewExpandedItem};
use crate::error::{Error, PersistenceSnafu};
use crate::extractors::{Path, UserContext};
use crate::related_data::WithGroupsAndUsers;
use crate::routes::ticket;
use crate::state::ApplicationState;
use axum::extract::State;
use cqrs_es::lifecycle::{LifecycleError, LifecycleViewState};
use cqrs_es::persist::ViewRepository;
use cqrs_es::AggregateError;
use snafu::ResultExt;
use tracing::error;

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
            return Err(Error::Group {
                source: AggregateError::UserError(LifecycleError::AggregateError(
                    GroupError::Forbidden,
                )),
            });
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
