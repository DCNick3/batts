use crate::auth::Authenticated;
use crate::domain::user::UserId;
use crate::error::ApiError;
use async_trait::async_trait;
use axum::http::StatusCode;
use cqrs_es::lifecycle::{LifecycleAggregate, LifecycleAggregateState, LifecycleEvent};
use cqrs_es::persist::{ViewContext, ViewRepository};
use cqrs_es::{AnyId, Id};
use cqrs_es::{DomainEvent, EventEnvelope, GenericView, Query, View};
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
pub enum GroupEvent {
    MemberAdded { member: UserId },
}

impl DomainEvent for GroupEvent {
    fn event_type(&self) -> String {
        match self {
            GroupEvent::MemberAdded { .. } => "MemberAdded".to_string(),
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
    type UpdateEvent = GroupEvent;
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
            vec![GroupEvent::MemberAdded { member: user_id }],
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
                Ok(vec![GroupEvent::MemberAdded { member: new_member }])
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
            GroupEvent::MemberAdded { member } => {
                self.members.insert(member);
            }
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct GroupViewContent {
    pub id: GroupId,
    pub title: String,
    pub members: BTreeSet<UserId>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum GroupView {
    #[default]
    NotCreated,
    Created(GroupViewContent),
}

impl GroupView {
    pub fn unwrap(self) -> GroupViewContent {
        match self {
            GroupView::NotCreated => panic!("Group not created"),
            GroupView::Created(content) => content,
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut GroupViewContent {
        match self {
            GroupView::NotCreated => panic!("Group not created"),
            GroupView::Created(content) => content,
        }
    }
}

impl View<GroupAggregate> for GroupView {}
impl GenericView<GroupAggregate> for GroupView {
    fn update(&mut self, event: &EventEnvelope<GroupAggregate>) {
        match &event.payload {
            LifecycleEvent::Created(GroupCreated { title }) => {
                let GroupView::NotCreated = self else {
                    panic!("Group already created");
                };
                *self = GroupView::Created(GroupViewContent {
                    id: event.aggregate_id,
                    title: title.clone(),
                    members: BTreeSet::new(),
                })
            }
            LifecycleEvent::Updated(GroupEvent::MemberAdded { member }) => {
                let this = self.unwrap_mut();
                this.members.insert(*member);
            }
            LifecycleEvent::Deleted => todo!(),
        }
    }
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct UserGroupsView {
    pub items: HashSet<GroupId>,
}

impl View<GroupAggregate> for UserGroupsView {}

pub struct UserGroupsQuery<R>
where
    R: ViewRepository<UserGroupsView, GroupAggregate>,
{
    view_repository: Arc<R>,
}

impl<R> UserGroupsQuery<R>
where
    R: ViewRepository<UserGroupsView, GroupAggregate>,
{
    pub fn new(view_repository: Arc<R>) -> Self {
        Self { view_repository }
    }
}

#[async_trait]
impl<R> Query<GroupAggregate> for UserGroupsQuery<R>
where
    R: ViewRepository<UserGroupsView, GroupAggregate>,
{
    async fn dispatch(&self, _aggregate_id: GroupId, events: &[EventEnvelope<GroupAggregate>]) {
        for event in events {
            if let LifecycleEvent::Updated(GroupEvent::MemberAdded { member }) = &event.payload {
                let user_id = member.0.to_string();

                let (mut view, context) = self
                    .view_repository
                    .load_with_context(&user_id)
                    .await
                    .unwrap()
                    .unwrap_or_else(|| (UserGroupsView::default(), ViewContext::new(user_id, 0)));

                view.items.insert(event.aggregate_id);
                self.view_repository
                    .update_view(view, context)
                    .await
                    .unwrap();
            }
        }
    }
}
