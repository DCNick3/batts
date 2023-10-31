use crate::api_result::ApiResult;
use crate::domain::group::GroupView;
use crate::domain::related_data::WithUsers;
use crate::domain::user::{
    CreateUser, IdentityView, UpdateUser, UserId, UserProfileView, UserView,
};
use crate::error::{Error, PersistenceSnafu, UserSnafu};
use crate::extractors::{Json, Path, UserContext};
use crate::state::ApplicationState;
use crate::view_repositry_ext::LifecycleViewRepositoryExt;
use axum::extract::State;
use cqrs_es::lifecycle::LifecycleCommand;
use cqrs_es::persist::ViewRepository;
use snafu::ResultExt;
use tracing::error;

pub async fn me_query(
    State(state): State<ApplicationState>,
    user_context: UserContext,
) -> ApiResult<UserView> {
    ApiResult::from_async_fn(|| async {
        state
            .user_view_repository
            .load_lifecycle(user_context.user_id())
            .await
            .context(PersistenceSnafu)?
            .ok_or(Error::NotFound)
    })
    .await
}

pub async fn profile_query(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
) -> ApiResult<UserProfileView> {
    ApiResult::from_async_fn(|| async {
        state
            .user_view_repository
            .load_lifecycle(id)
            .await
            .context(PersistenceSnafu)?
            .map(UserView::profile)
            .ok_or(Error::NotFound)
    })
    .await
}

pub async fn groups_query(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
) -> ApiResult<WithUsers<Vec<GroupView>>> {
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

        let results = results
            .into_iter()
            .flat_map(|view| {
                let group = view.unwrap().into_created();
                if group.is_none() {
                    error!("Group not found");
                }
                group
            })
            .collect();

        WithUsers::new(state.user_view_repository.as_ref(), results).await
    })
    .await
}

pub async fn internal_query(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
) -> ApiResult<UserView> {
    ApiResult::from_async_fn(|| async {
        state
            .user_view_repository
            .load_lifecycle(id)
            .await
            .context(PersistenceSnafu)?
            .ok_or(Error::NotFound)
    })
    .await
}

pub async fn internal_create_command(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
    Json(command): Json<CreateUser>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .user_cqrs
            .execute(id, LifecycleCommand::Create(command))
            .await
            .context(UserSnafu),
    )
}

pub async fn internal_update_command(
    State(state): State<ApplicationState>,
    Path(id): Path<UserId>,
    Json(command): Json<UpdateUser>,
) -> ApiResult {
    ApiResult::from_result(
        state
            .user_cqrs
            .execute(id, LifecycleCommand::Update(command))
            .await
            .context(UserSnafu),
    )
}

pub async fn internal_identity(
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
