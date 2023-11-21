use crate::domain::group::{GroupId, GroupProfileView, GroupView};
use crate::domain::user::{UserId, UserProfileView, UserView};
use crate::error::{Error, PersistenceSnafu};
use crate::state::CqrsState;
use crate::view_repositry_ext::LifecycleViewRepositoryExt;
use async_trait::async_trait;
pub use batts_derive::CollectIds;
use cqrs_es::lifecycle::{LifecycleAggregate, LifecycleCommand, LifecycleView, LifecycleViewState};
use cqrs_es::persist::ViewRepository;
use cqrs_es::AnyId;
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use ts_rs::TS;

pub trait CollectIds<Id: AnyId> {
    fn collect_ids(&self, target: &mut IndexSet<Id>);
}

macro_rules! noop_impl {
    ($($ty:ty),*) => {
        $(
            impl<Id: AnyId> CollectIds<Id> for $ty {
                fn collect_ids(&self, _: &mut IndexSet<Id>) {}
            }
        )*
    };
}

noop_impl!((), i32, i64, String);
noop_impl!(chrono::DateTime<chrono::Utc>);

impl<Id: AnyId, T: CollectIds<Id>> CollectIds<Id> for Option<T> {
    fn collect_ids(&self, target: &mut IndexSet<Id>) {
        if let Some(item) = self {
            item.collect_ids(target);
        }
    }
}

impl<Id: AnyId, T: CollectIds<Id>> CollectIds<Id> for Vec<T> {
    fn collect_ids(&self, user_ids: &mut IndexSet<Id>) {
        for item in self {
            item.collect_ids(user_ids);
        }
    }
}
impl<Id: AnyId, T: CollectIds<Id>> CollectIds<Id> for IndexSet<T> {
    fn collect_ids(&self, target: &mut IndexSet<Id>) {
        for item in self {
            item.collect_ids(target);
        }
    }
}

impl<Id: AnyId, A: LifecycleAggregate> CollectIds<Id> for LifecycleCommand<A>
where
    <A as LifecycleAggregate>::CreateCommand: CollectIds<Id>,
    <A as LifecycleAggregate>::UpdateCommand: CollectIds<Id>,
{
    fn collect_ids(&self, target: &mut IndexSet<Id>) {
        match self {
            LifecycleCommand::Create(cmd) => cmd.collect_ids(target),
            LifecycleCommand::Update(cmd) => cmd.collect_ids(target),
            // I don't think the delete command will have any arguments, so don't bother for now
            LifecycleCommand::Delete(_) => {}
        }
    }
}

async fn load_all<R, V, I>(view_repository: &R, ids: I) -> Result<Vec<V>, Error>
where
    R: ViewRepository<LifecycleViewState<V>>,
    V: LifecycleView,
    I: Iterator<Item = <V::Aggregate as LifecycleAggregate>::Id>,
{
    futures_util::future::join_all(ids.map(|id| view_repository.load_lifecycle(id)))
        .await
        .into_iter()
        .map(|v| {
            v.context(PersistenceSnafu)
                .and_then(|v| v.ok_or(Error::RelatedItemNotFound))
        })
        .collect::<Result<Vec<_>, _>>()
}

async fn retrieve_users<R>(
    view_repository: &R,
    user_ids: IndexSet<UserId>,
) -> Result<IndexMap<UserId, UserProfileView>, Error>
where
    R: ViewRepository<LifecycleViewState<UserView>>,
{
    Ok(load_all(view_repository, user_ids.into_iter())
        .await?
        .into_iter()
        .map(|user| (user.id, user.profile()))
        .collect())
}

async fn retrieve_groups<R>(
    view_repository: &R,
    group_ids: IndexSet<GroupId>,
) -> Result<IndexMap<GroupId, GroupProfileView>, Error>
where
    R: ViewRepository<LifecycleViewState<GroupView>>,
{
    Ok(load_all(view_repository, group_ids.into_iter())
        .await?
        .into_iter()
        .map(|group| (group.id, group.profile()))
        .collect())
}

#[async_trait]
pub trait ViewWithRelated: Sized {
    type View;

    // sad that we have to pass the whole state tbh...
    async fn new(state: &CqrsState, payload: Self::View) -> Result<Self, Error>;
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct WithUsers<T> {
    pub users: IndexMap<UserId, UserProfileView>,
    pub payload: T,
}

#[async_trait]
impl<T: CollectIds<UserId> + Send> ViewWithRelated for WithUsers<T> {
    type View = T;

    async fn new(state: &CqrsState, payload: Self::View) -> Result<Self, Error> {
        let mut user_ids = IndexSet::new();
        payload.collect_ids(&mut user_ids);

        let users = retrieve_users(state.user_view_repository.as_ref(), user_ids).await?;

        Ok(Self { users, payload })
    }
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct WithGroups<T> {
    pub groups: IndexMap<GroupId, GroupProfileView>,
    pub payload: T,
}

#[async_trait]
impl<T: CollectIds<GroupId> + Send> ViewWithRelated for WithGroups<T> {
    type View = T;

    async fn new(state: &CqrsState, payload: Self::View) -> Result<Self, Error> {
        let mut group_ids = IndexSet::new();
        payload.collect_ids(&mut group_ids);

        let groups = retrieve_groups(state.group_view_repository.as_ref(), group_ids).await?;

        Ok(Self { groups, payload })
    }
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct WithGroupsAndUsers<T> {
    pub groups: IndexMap<GroupId, GroupProfileView>,
    pub users: IndexMap<UserId, UserProfileView>,
    pub payload: T,
}

#[async_trait]
impl<T: CollectIds<UserId> + CollectIds<GroupId> + Send> ViewWithRelated for WithGroupsAndUsers<T> {
    type View = T;

    async fn new(state: &CqrsState, payload: Self::View) -> Result<Self, Error> {
        let mut user_ids = IndexSet::new();
        let mut group_ids = IndexSet::new();
        payload.collect_ids(&mut user_ids);
        payload.collect_ids(&mut group_ids);

        let users = retrieve_users(state.user_view_repository.as_ref(), user_ids).await?;
        let groups = retrieve_groups(state.group_view_repository.as_ref(), group_ids).await?;

        Ok(Self {
            users,
            groups,
            payload,
        })
    }
}
