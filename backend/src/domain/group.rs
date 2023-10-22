use crate::auth::Authenticated;
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::id::Id;
use async_trait::async_trait;
use axum::http::StatusCode;
use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, View};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::{BTreeSet, HashSet};
use std::str::FromStr;
use ts_rs::TS;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct GroupId(pub Id);

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
    Create(CreateGroup),
    AddMember(AddGroupMember),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupEvent {
    Created { name: String },
    MemberAdded { member: UserId },
}

impl DomainEvent for GroupEvent {
    fn event_type(&self) -> String {
        match self {
            GroupEvent::Created { .. } => "Created".to_string(),
            GroupEvent::MemberAdded { .. } => "MemberAdded".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Group {
    #[default]
    NotCreated,
    Created(GroupContent),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GroupContent {
    pub title: String,
    pub members: HashSet<UserId>,
}

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
impl Aggregate for Group {
    type Command = Authenticated<GroupCommand>;
    type Event = GroupEvent;
    type Error = GroupError;
    type Services = ();

    fn aggregate_type() -> String {
        "Group".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        let performer = command.user_id;

        match command.payload {
            GroupCommand::Create(CreateGroup { title }) => {
                let Group::NotCreated = self else {
                    return Err(GroupError::AlreadyExists);
                };
                Ok(vec![
                    GroupEvent::Created { name: title },
                    GroupEvent::MemberAdded { member: performer },
                ])
            }
            GroupCommand::AddMember(AddGroupMember { new_member }) => {
                let Group::Created(content) = self else {
                    return Err(GroupError::DoesNotExist);
                };
                if !content.members.contains(&performer) {
                    return Err(GroupError::Forbidden);
                }
                if content.members.contains(&new_member) {
                    return Ok(vec![]);
                }
                Ok(vec![GroupEvent::MemberAdded { member: new_member }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            GroupEvent::Created { name } => {
                let Group::NotCreated = self else {
                    panic!("User already created");
                };
                *self = Group::Created(GroupContent {
                    title: name,
                    members: HashSet::new(),
                })
            }
            GroupEvent::MemberAdded { member } => {
                let Group::Created(content) = self else {
                    panic!("User not created");
                };
                content.members.insert(member);
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

impl View<Group> for GroupView {
    fn update(&mut self, event: &EventEnvelope<Group>) {
        match &event.payload {
            GroupEvent::Created { name } => {
                let GroupView::NotCreated = self else {
                    panic!("Group already created");
                };
                *self = GroupView::Created(GroupViewContent {
                    id: GroupId(Id::from_str(&event.aggregate_id).unwrap()),
                    title: name.clone(),
                    members: BTreeSet::new(),
                })
            }
            GroupEvent::MemberAdded { member } => {
                let this = self.unwrap_mut();
                this.members.insert(*member);
            }
        }
    }
}