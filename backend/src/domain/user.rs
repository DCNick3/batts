use crate::error::ApiError;
use crate::id::Id;
use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use cqrs_es::persist::{ViewContext, ViewRepository};
use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, Query, View};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use tracing::warn;
use ts_rs::TS;

#[derive(
    Default, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, TS, Serialize, Deserialize,
)]
#[ts(export)]
pub struct UserId(pub Id);

#[derive(Debug, TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export)]
pub enum UserCommand {
    Create { profile: ExternalUserProfile },
    AddIdentity { profile: ExternalUserProfile },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserEvent {
    Created { name: String },
    IdentityAdded { profile: ExternalUserProfile },
}

impl DomainEvent for UserEvent {
    fn event_type(&self) -> String {
        match self {
            UserEvent::Created { .. } => "Created".to_string(),
            UserEvent::IdentityAdded { .. } => "IdentityAdded".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub enum User {
    #[default]
    NotCreated,
    Created(UserContent),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserContent {
    pub name: String,
    pub identities: UserIdentities,
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct UserIdentities {
    pub telegram: Option<TelegramProfile>,
    pub university: Option<UniversityProfile>,
}

impl UserIdentities {
    pub fn get_identities(&self) -> Vec<ExternalUserIdentity> {
        let mut identities = Vec::new();
        if let Some(telegram) = &self.telegram {
            identities.push(ExternalUserIdentity::Telegram(telegram.id));
        }
        if let Some(university) = &self.university {
            identities.push(ExternalUserIdentity::University(university.email.clone()));
        }
        identities
    }

    pub fn can_add_identity(&self, profile: &ExternalUserProfile) -> bool {
        match profile {
            ExternalUserProfile::Telegram(_) => self.telegram.is_none(),
            ExternalUserProfile::University(_) => self.university.is_none(),
        }
    }

    /// Add a new identity to the user.
    ///
    /// NOTE: this method does not check if the identity already exists, overwriting it.
    pub fn add_identity(&mut self, profile: ExternalUserProfile) {
        match profile {
            ExternalUserProfile::Telegram(profile) => {
                self.telegram = Some(profile);
            }
            ExternalUserProfile::University(profile) => {
                self.university = Some(profile);
            }
        }
    }
}

#[derive(Snafu, Debug)]
pub enum UserError {
    /// The user with the provided id already exists.
    AlreadyExists,
    /// The user with the provided id does not exist.
    DoesNotExist,
    /// The user already has a profile for the provided identity provider.
    IdentityExists,
    /// Some user already has associated the provided identity with their account.
    IdentityUsed,
}

impl ApiError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::AlreadyExists => StatusCode::BAD_REQUEST,
            UserError::DoesNotExist => StatusCode::NOT_FOUND,
            UserError::IdentityExists => StatusCode::BAD_REQUEST,
            UserError::IdentityUsed => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExternalUserIdentity {
    Telegram(i64),
    University(String),
}

impl Display for ExternalUserIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalUserIdentity::Telegram(id) => write!(f, "telegram-{}", id),
            ExternalUserIdentity::University(email) => write!(f, "university-{}", email),
        }
    }
}

#[derive(Debug, Clone, TS, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[ts(export)]
pub enum ExternalUserProfile {
    Telegram(TelegramProfile),
    University(UniversityProfile),
}

impl ExternalUserProfile {
    pub fn name(&self) -> String {
        match self {
            ExternalUserProfile::Telegram(profile) => {
                format!("{} {}", profile.first_name, profile.last_name)
            }
            ExternalUserProfile::University(profile) => profile.commonname.clone(),
        }
    }

    pub fn identity(&self) -> ExternalUserIdentity {
        match self {
            ExternalUserProfile::Telegram(profile) => ExternalUserIdentity::Telegram(profile.id),
            ExternalUserProfile::University(profile) => {
                ExternalUserIdentity::University(profile.email.clone())
            }
        }
    }
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize, PartialEq)]
#[ts(export)]
pub struct TelegramProfile {
    #[ts(type = "number")]
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize, PartialEq)]
#[ts(export)]
pub struct TelegramLoginData {
    #[serde(flatten)]
    pub profile: TelegramProfile,
    #[ts(type = "number")]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub auth_date: DateTime<Utc>,
    pub hash: String,
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize, PartialEq)]
#[ts(export)]
pub struct UniversityProfile {
    pub email: String,
    pub commonname: String,
    pub family_name: String,
    pub given_name: String,
}

pub struct UserServices {
    pub user_identity_view_repository: Arc<dyn ViewRepository<IdentityView, User>>,
}

#[async_trait]
impl Aggregate for User {
    type Command = UserCommand;
    type Event = UserEvent;
    type Error = UserError;
    type Services = UserServices;

    fn aggregate_type() -> String {
        "User".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            UserCommand::Create { profile } => {
                let User::NotCreated = self else {
                    return Err(UserError::AlreadyExists);
                };
                Ok(vec![
                    UserEvent::Created {
                        name: profile.name(),
                    },
                    UserEvent::IdentityAdded { profile },
                ])
            }
            UserCommand::AddIdentity { profile } => {
                let User::Created(user) = self else {
                    return Err(UserError::DoesNotExist);
                };
                if !user.identities.can_add_identity(&profile) {
                    return Err(UserError::IdentityExists);
                }
                if let Some(_) = service
                    .user_identity_view_repository
                    .load(&profile.identity().to_string())
                    .await
                    .unwrap()
                {
                    return Err(UserError::IdentityUsed);
                }
                Ok(vec![UserEvent::IdentityAdded { profile }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            UserEvent::Created { name } => {
                let User::NotCreated = self else {
                    panic!("User already created");
                };
                *self = User::Created(UserContent {
                    name,
                    identities: Default::default(),
                });
            }
            UserEvent::IdentityAdded { profile } => {
                let User::Created(user) = self else {
                    panic!("User not created");
                };
                user.identities.add_identity(profile);
            }
        }
    }
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct UserView {
    pub id: UserId,
    pub name: String,
    pub identities: UserIdentities,
}

impl UserView {
    pub fn profile(self) -> UserProfileView {
        UserProfileView {
            id: self.id,
            name: self.name,
        }
    }
}

impl View<User> for UserView {
    fn update(&mut self, event: &EventEnvelope<User>) {
        match &event.payload {
            UserEvent::Created { name } => {
                self.id = UserId(Id::from_str(&event.aggregate_id).unwrap());
                self.name = name.clone();
            }
            UserEvent::IdentityAdded { profile } => {
                self.identities.add_identity(profile.clone());
            }
        }
    }
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct UserProfileView {
    pub id: UserId,
    pub name: String,
}

#[derive(Default, Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct IdentityView {
    pub user_id: UserId,
}

impl View<User> for IdentityView {
    fn update(&mut self, event: &EventEnvelope<User>) {
        let id = UserId(Id::from_str(&event.aggregate_id).unwrap());
        self.user_id = id;
    }
}

pub struct IdentityQuery<R>
where
    R: ViewRepository<IdentityView, User>,
{
    view_repository: Arc<R>,
}

impl<R> IdentityQuery<R>
where
    R: ViewRepository<IdentityView, User>,
{
    pub fn new(view_repository: Arc<R>) -> Self {
        Self { view_repository }
    }
}

#[async_trait]
impl<R> Query<User> for IdentityQuery<R>
where
    R: ViewRepository<IdentityView, User>,
{
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<User>]) {
        let user_id = UserId(Id::from_str(aggregate_id).unwrap());

        for event in events {
            if let UserEvent::IdentityAdded { profile } = &event.payload {
                let identity_id = profile.identity().to_string();
                match self
                    .view_repository
                    .load_with_context(&identity_id)
                    .await
                    .unwrap()
                {
                    Some((mut view, context)) => {
                        warn!("Identity already exists, reassigning to another user");
                        view.update(event);
                        self.view_repository
                            .update_view(view, context)
                            .await
                            .unwrap();
                    }
                    None => {
                        let view = IdentityView { user_id };
                        let context = ViewContext::new(identity_id, 0);
                        self.view_repository
                            .update_view(view, context)
                            .await
                            .unwrap();
                    }
                }
            }
        }
    }
}
