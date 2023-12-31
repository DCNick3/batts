use crate::api_result::ApiResult;
use crate::domain::ticket::{TicketListingView, TicketListingViewExpandedItem};
use crate::error::{Error, PersistenceSnafu};
use crate::extractors::UserContext;
use crate::related_data::{ViewWithRelated as _, WithGroupsAndUsers};
use crate::state::ApplicationState;
use crate::view_repositry_ext::LifecycleViewRepositoryExt;
use axum::extract::State;
use cqrs_es::persist::ViewRepository;
use itertools::Itertools;
use snafu::ResultExt;

pub async fn expand_ticket_listing_view(
    state: ApplicationState,
    ticket_view: TicketListingView,
) -> Result<WithGroupsAndUsers<Vec<TicketListingViewExpandedItem>>, Error> {
    let results = futures_util::future::join_all(
        ticket_view
            .items
            .iter()
            .map(|id| async { state.cqrs.ticket_view_repository.load_lifecycle(*id).await }),
    )
    .await;
    let results = results
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .context(PersistenceSnafu)?;

    let results = results
        .into_iter()
        .map(|view| {
            let view = view.unwrap();
            TicketListingViewExpandedItem {
                id: view.id,
                destination: view.destination,
                owner: view.owner,
                assignee: view.assignee,
                title: view.title,
                status: view.status,
                latest_update: view.latest_update,
            }
        })
        .sorted_by_key(|v| v.latest_update)
        .rev()
        .collect();

    WithGroupsAndUsers::new(&state.cqrs, results).await
}

pub async fn assignee_listing_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
) -> ApiResult<WithGroupsAndUsers<Vec<TicketListingViewExpandedItem>>> {
    ApiResult::from_async_fn(|| async {
        let view = state
            .cqrs
            .ticket_assignee_listing_view_repository
            .load(&user_context.user_id().0.to_string())
            .await
            .context(PersistenceSnafu)?
            .unwrap_or_default();

        expand_ticket_listing_view(state, view).await
    })
    .await
}

pub async fn owned_listing_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
) -> ApiResult<WithGroupsAndUsers<Vec<TicketListingViewExpandedItem>>> {
    ApiResult::from_async_fn(|| async {
        let view = state
            .cqrs
            .ticket_owner_listing_view_repository
            .load(&user_context.user_id().0.to_string())
            .await
            .context(PersistenceSnafu)?
            .unwrap_or_default();

        expand_ticket_listing_view(state, view).await
    })
    .await
}
