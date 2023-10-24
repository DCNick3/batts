use crate::auth::Authenticated;
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::view_repositry_ext::ViewRepositoryExt;
use async_trait::async_trait;
use axum::http::StatusCode;
use cqrs_es::lifecycle::{
    CreateEnvelope, LifecycleAggregate, LifecycleAggregateState, LifecycleEvent, LifecycleView,
    UpdateEnvelope,
};
use cqrs_es::persist::ViewRepository;
use cqrs_es::{AnyId, Id};
use cqrs_es::{DomainEvent, EventEnvelope, Query, View};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, TS, Serialize, Deserialize)]
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
#[serde(tag = "type")]
#[ts(export)]
pub enum GroupCommand {
    AddMember(AddGroupMember),
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
    MemberAdded { member: UserId },
}

impl DomainEvent for GroupUpdated {
    fn event_type(&self) -> String {
        match self {
            GroupUpdated::MemberAdded { .. } => "MemberAdded".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub title: String,
    pub members: HashSet<UserId>,
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
    type UpdateCommand = Authenticated<GroupCommand>;
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
            user_id,
            payload: CreateGroup { title },
        }: Self::CreateCommand,
        _service: &Self::Services,
    ) -> Result<(Self::CreateEvent, Vec<Self::UpdateEvent>), Self::Error> {
        Ok((
            GroupCreated { title },
            vec![GroupUpdated::MemberAdded { member: user_id }],
        ))
    }

    async fn handle(
        &self,
        command: Self::UpdateCommand,
        _service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error> {
        let performer = command.user_id;

        match command.payload {
            GroupCommand::AddMember(AddGroupMember { new_member }) => {
                if !self.members.contains(&performer) {
                    return Err(GroupError::Forbidden);
                }
                if self.members.contains(&new_member) {
                    return Ok(vec![]);
                }
                Ok(vec![GroupUpdated::MemberAdded { member: new_member }])
            }
        }
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
            members: HashSet::new(),
        }
    }

    fn apply(&mut self, event: Self::UpdateEvent) {
        match event {
            GroupUpdated::MemberAdded { member } => {
                self.members.insert(member);
            }
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct GroupView {
    pub id: GroupId,
    pub title: String,
    pub members: BTreeSet<UserId>,
}

impl LifecycleView for GroupView {
    type Aggregate = Group;

    fn create(event: CreateEnvelope<'_, Self::Aggregate>) -> Self {
        let GroupCreated { title } = event.payload;
        Self {
            id: event.aggregate_id,
            title: title.clone(),
            members: BTreeSet::new(),
        }
    }

    fn update(&mut self, event: UpdateEnvelope<'_, Self::Aggregate>) {
        match *event.payload {
            GroupUpdated::MemberAdded { member } => {
                self.members.insert(member);
            }
        }
    }
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct UserGroupsView {
    pub items: HashSet<GroupId>,
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
    async fn dispatch(
        &self,
        _aggregate_id: GroupId,
        events: &[EventEnvelope<GroupId, LifecycleEvent<GroupCreated, GroupUpdated>>],
    ) {
        for event in events {
            if let LifecycleEvent::Updated(GroupUpdated::MemberAdded { member }) = &event.payload {
                let user_id = member.0.to_string();

                self.view_repository
                    .load_modify_update_default(&user_id, |view| {
                        view.items.insert(event.aggregate_id);
                    })
                    .await
                    .unwrap();
            }
        }
    }
}
