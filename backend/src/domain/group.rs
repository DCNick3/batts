use crate::auth::Authenticated;
use crate::domain::user::UserId;
use crate::domain::CollectIds;
use crate::error::ApiError;
use crate::view_repositry_ext::ViewRepositoryExt;
use async_trait::async_trait;
use axum::http::StatusCode;
use cqrs_es::lifecycle::{
    CreateEnvelope, LifecycleAggregate, LifecycleAggregateState, LifecycleEnvelope, LifecycleEvent,
    LifecycleView, UpdateEnvelope,
};
use cqrs_es::persist::ViewRepository;
use cqrs_es::{AnyId, Id};
use cqrs_es::{DomainEvent, Query, View};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct GroupId(pub Id);

impl AnyId for GroupId {
    fn from_id(id: Id) -> Self {
        Self(id)
    }

    fn id(&self) -> Id {
        self.0
    }
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct CreateGroup {
    pub title: String,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct AddGroupMember {
    pub new_member: UserId,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct RemoveGroupMember {
    pub removed_member: UserId,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct ChangeGroupTitle {
    pub new_title: String,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export)]
pub enum UpdateGroup {
    AddMember(AddGroupMember),
    RemoveMember(RemoveGroupMember),
    ChangeTitle(ChangeGroupTitle),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupCreated {
    title: String,
}

impl DomainEvent for GroupCreated {
    fn event_type(&self) -> String {
        "".to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupUpdated {
    MemberAdded {
        performer: UserId,
        member: UserId,
    },
    MemberRemoved {
        performer: UserId,
        member: UserId,
    },
    TitleChanged {
        performer: UserId,
        old_title: String,
        new_title: String,
    },
}

impl DomainEvent for GroupUpdated {
    fn event_type(&self) -> String {
        match self {
            GroupUpdated::MemberAdded { .. } => "MemberAdded".to_string(),
            GroupUpdated::MemberRemoved { .. } => "MemberRemoved".to_string(),
            GroupUpdated::TitleChanged { .. } => "TitleChanged".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub title: String,
    pub members: IndexSet<UserId>,
}

impl Group {
    fn check_access(&self, user_id: UserId) -> Result<(), GroupError> {
        if !self.members.contains(&user_id) {
            return Err(GroupError::Forbidden);
        }
        Ok(())
    }
}

pub type GroupAggregate = LifecycleAggregateState<Group>;

#[derive(Snafu, Debug)]
pub enum GroupError {
    /// Group already exists
    AlreadyExists,
    /// Group does not exist
    DoesNotExist,
    /// This user cannot perform this action
    Forbidden,
}

impl ApiError for GroupError {
    fn status_code(&self) -> StatusCode {
        match self {
            GroupError::AlreadyExists => StatusCode::BAD_REQUEST,
            GroupError::DoesNotExist => StatusCode::NOT_FOUND,
            GroupError::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}

#[async_trait]
impl LifecycleAggregate for Group {
    type Id = GroupId;
    type CreateCommand = Authenticated<CreateGroup>;
    type UpdateCommand = Authenticated<UpdateGroup>;
    type DeleteCommand = ();
    type CreateEvent = GroupCreated;
    type UpdateEvent = GroupUpdated;
    type Error = GroupError;
    type Services = ();

    fn aggregate_type() -> String {
        "Group".to_string()
    }

    async fn handle_create(
        Authenticated {
            user_id: performer,
            payload: CreateGroup { title },
        }: Self::CreateCommand,
        _service: &Self::Services,
    ) -> Result<(Self::CreateEvent, Vec<Self::UpdateEvent>), Self::Error> {
        Ok((
            GroupCreated { title },
            vec![GroupUpdated::MemberAdded {
                performer,
                member: performer,
            }],
        ))
    }

    async fn handle(
        &self,
        command: Self::UpdateCommand,
        _service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error> {
        let mut events = Vec::new();
        let performer = command.user_id;

        match command.payload {
            UpdateGroup::AddMember(AddGroupMember { new_member }) => {
                self.check_access(performer)?;
                if !self.members.contains(&new_member) {
                    events.push(GroupUpdated::MemberAdded {
                        performer,
                        member: new_member,
                    });
                }
            }
            UpdateGroup::RemoveMember(RemoveGroupMember { removed_member }) => {
                self.check_access(performer)?;
                if self.members.contains(&removed_member) {
                    events.push(GroupUpdated::MemberRemoved {
                        performer,
                        member: removed_member,
                    });
                }
            }
            UpdateGroup::ChangeTitle(ChangeGroupTitle { new_title }) => {
                self.check_access(performer)?;
                if self.title != new_title {
                    events.push(GroupUpdated::TitleChanged {
                        performer,
                        old_title: self.title.clone(),
                        new_title,
                    })
                }
            }
        }

        Ok(events)
    }

    async fn handle_delete(
        &self,
        _command: Self::DeleteCommand,
        _service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error> {
        todo!()
    }

    fn apply_create(GroupCreated { title }: Self::CreateEvent) -> Self {
        Self {
            title,
            members: IndexSet::new(),
        }
    }

    fn apply(&mut self, event: Self::UpdateEvent) {
        match event {
            GroupUpdated::MemberAdded { member, .. } => {
                self.members.insert(member);
            }
            GroupUpdated::MemberRemoved { member, .. } => {
                self.members.shift_remove(&member);
            }
            GroupUpdated::TitleChanged { new_title, .. } => {
                self.title = new_title;
            }
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct GroupView {
    pub id: GroupId,
    pub title: String,
    pub members: IndexSet<UserId>,
}

impl GroupView {
    pub fn profile(&self) -> GroupProfileView {
        GroupProfileView {
            id: self.id,
            title: self.title.clone(),
        }
    }
}

impl LifecycleView for GroupView {
    type Aggregate = Group;

    fn create(event: CreateEnvelope<'_, Self::Aggregate>) -> Self {
        let GroupCreated { title } = event.payload;
        Self {
            id: event.aggregate_id,
            title: title.clone(),
            members: IndexSet::new(),
        }
    }

    fn update(&mut self, event: UpdateEnvelope<'_, Self::Aggregate>) {
        match *event.payload {
            GroupUpdated::MemberAdded { member, .. } => {
                self.members.insert(member);
            }
            GroupUpdated::MemberRemoved { member, .. } => {
                self.members.remove(&member);
            }
            GroupUpdated::TitleChanged { ref new_title, .. } => {
                self.title = new_title.clone();
            }
        }
    }
}

impl CollectIds<UserId> for GroupView {
    fn collect_ids(&self, user_ids: &mut IndexSet<UserId>) {
        user_ids.extend(self.members.iter().cloned());
    }
}

impl CollectIds<GroupId> for GroupView {
    fn collect_ids(&self, group_ids: &mut IndexSet<GroupId>) {
        group_ids.insert(self.id);
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct GroupProfileView {
    pub id: GroupId,
    pub title: String,
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct UserGroupsView {
    pub items: IndexSet<GroupId>,
}

impl View for UserGroupsView {
    type Aggregate = GroupAggregate;
}

pub struct UserGroupsQuery<R>
where
    R: ViewRepository<UserGroupsView>,
{
    view_repository: Arc<R>,
}

impl<R> UserGroupsQuery<R>
where
    R: ViewRepository<UserGroupsView>,
{
    pub fn new(view_repository: Arc<R>) -> Self {
        Self { view_repository }
    }
}

#[async_trait]
impl<R> Query<GroupAggregate> for UserGroupsQuery<R>
where
    R: ViewRepository<UserGroupsView>,
{
    async fn dispatch(&self, _aggregate_id: GroupId, events: &[LifecycleEnvelope<Group>]) {
        for event in events {
            let group_id = event.aggregate_id;
            if let LifecycleEvent::Updated(
                event @ (GroupUpdated::MemberAdded { member, .. }
                | GroupUpdated::MemberRemoved { member, .. }),
            ) = &event.payload
            {
                let user_id = member.0.to_string();

                self.view_repository
                    .load_modify_update_default(&user_id, |view| match event {
                        GroupUpdated::MemberAdded { .. } => {
                            view.items.insert(group_id);
                        }
                        GroupUpdated::MemberRemoved { .. } => {
                            view.items.shift_remove(&group_id);
                        }
                        _ => unreachable!(),
                    })
                    .await
                    .unwrap();
            }
        }
    }
}
