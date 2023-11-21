use crate::domain::group::{GroupId, GroupProfileView, GroupView};
use crate::domain::user::{UserId, UserProfileView, UserView};
use crate::error::{Error, PersistenceSnafu};
use crate::view_repositry_ext::LifecycleViewRepositoryExt;
pub use batts_derive::CollectIds;
use cqrs_es::lifecycle::{LifecycleAggregate, LifecycleView, LifecycleViewState};
use cqrs_es::persist::ViewRepository;
use cqrs_es::AnyId;
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use ts_rs::TS;

pub trait CollectIds<Id: AnyId> {
    fn collect_ids(&self, target: &mut IndexSet<Id>);
}

impl<Id: AnyId> CollectIds<Id> for () {
    fn collect_ids(&self, _: &mut IndexSet<Id>) {}
}

impl<Id: AnyId> CollectIds<Id> for String {
    fn collect_ids(&self, _: &mut IndexSet<Id>) {}
}
impl<Id: AnyId> CollectIds<Id> for chrono::DateTime<chrono::Utc> {
    fn collect_ids(&self, _: &mut IndexSet<Id>) {}
}

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

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct WithUsers<T> {
    pub users: IndexMap<UserId, UserProfileView>,
    pub payload: T,
}

impl<T: CollectIds<UserId>> WithUsers<T> {
    pub async fn new<R>(view_repository: &R, payload: T) -> Result<Self, Error>
    where
        R: ViewRepository<LifecycleViewState<UserView>>,
    {
        let mut user_ids = IndexSet::new();
        payload.collect_ids(&mut user_ids);

        let users = retrieve_users(view_repository, user_ids).await?;

        Ok(Self { users, payload })
    }
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct WithGroups<T> {
    pub groups: IndexMap<GroupId, GroupProfileView>,
    pub payload: T,
}

impl<T: CollectIds<GroupId>> WithGroups<T> {
    pub async fn new<R>(view_repository: &R, payload: T) -> Result<Self, Error>
    where
        R: ViewRepository<LifecycleViewState<GroupView>>,
    {
        let mut group_ids = IndexSet::new();
        payload.collect_ids(&mut group_ids);

        let groups = retrieve_groups(view_repository, group_ids).await?;

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

impl<T: CollectIds<UserId> + CollectIds<GroupId>> WithGroupsAndUsers<T> {
    pub async fn new<UR, GR>(
        user_view_repository: &UR,
        group_view_repository: &GR,
        payload: T,
    ) -> Result<Self, Error>
    where
        UR: ViewRepository<LifecycleViewState<UserView>>,
        GR: ViewRepository<LifecycleViewState<GroupView>>,
    {
        let mut user_ids = IndexSet::new();
        let mut group_ids = IndexSet::new();
        payload.collect_ids(&mut user_ids);
        payload.collect_ids(&mut group_ids);

        let users = retrieve_users(user_view_repository, user_ids).await?;
        let groups = retrieve_groups(group_view_repository, group_ids).await?;

        Ok(Self {
            users,
            groups,
            payload,
        })
    }
}
