use crate::error::ApiError;
use crate::view_repositry_ext::ViewRepositoryExt;
use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use cqrs_es::lifecycle::{
    CreateEnvelope, LifecycleAggregate, LifecycleAggregateState, LifecycleEnvelope, LifecycleEvent,
    LifecycleView, UpdateEnvelope,
};
use cqrs_es::persist::ViewRepository;
use cqrs_es::{AnyId, Id};
use cqrs_es::{DomainEvent, Query, View};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Display;
use std::sync::Arc;
use tracing::warn;
use ts_rs::TS;

#[derive(
    Default, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, TS, Serialize, Deserialize,
)]
#[ts(export)]
pub struct UserId(pub Id);

impl AnyId for UserId {
    fn from_id(id: Id) -> Self {
        Self(id)
    }

    fn id(&self) -> Id {
        self.0
    }
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct CreateUser {
    pub profile: ExternalUserProfile,
}

#[derive(Debug, TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export)]
pub enum UpdateUser {
    AddIdentity { profile: ExternalUserProfile },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCreated {
    name: String,
}

impl DomainEvent for UserCreated {
    fn event_type(&self) -> String {
        "".to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserUpdated {
    IdentityAdded { profile: ExternalUserProfile },
}

impl DomainEvent for UserUpdated {
    fn event_type(&self) -> String {
        match self {
            UserUpdated::IdentityAdded { .. } => "IdentityAdded".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub identities: UserIdentities,
}

pub type UserAggregate = LifecycleAggregateState<User>;

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
                match (&profile.first_name, &profile.last_name) {
                    (first_name, Some(last_name)) => {
                        format!("{} {}", first_name, last_name)
                    }
                    (first_name, None) => first_name.to_string(),
                }
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
    pub last_name: Option<String>,
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
    pub user_identity_view_repository: Arc<dyn ViewRepository<IdentityView>>,
}

#[async_trait]
impl LifecycleAggregate for User {
    type Id = UserId;
    type CreateCommand = CreateUser;
    type UpdateCommand = UpdateUser;
    type DeleteCommand = ();
    type CreateEvent = UserCreated;
    type UpdateEvent = UserUpdated;
    type Error = UserError;
    type Services = UserServices;

    fn aggregate_type() -> String {
        "User".to_string()
    }

    async fn handle_create(
        CreateUser { profile }: Self::CreateCommand,
        _service: &Self::Services,
    ) -> Result<(Self::CreateEvent, Vec<Self::UpdateEvent>), Self::Error> {
        Ok((
            UserCreated {
                name: profile.name(),
            },
            vec![UserUpdated::IdentityAdded { profile }],
        ))
    }

    async fn handle(
        &self,
        command: Self::UpdateCommand,
        service: &Self::Services,
    ) -> Result<Vec<Self::UpdateEvent>, Self::Error> {
        match command {
            UpdateUser::AddIdentity { profile } => {
                if !self.identities.can_add_identity(&profile) {
                    return Err(UserError::IdentityExists);
                }
                if service
                    .user_identity_view_repository
                    .load(&profile.identity().to_string())
                    .await
                    .unwrap()
                    .is_some()
                {
                    return Err(UserError::IdentityUsed);
                }
                Ok(vec![UserUpdated::IdentityAdded { profile }])
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

    fn apply_create(UserCreated { name }: Self::CreateEvent) -> Self {
        Self {
            name,
            identities: Default::default(),
        }
    }

    fn apply(&mut self, event: Self::UpdateEvent) {
        match event {
            UserUpdated::IdentityAdded { profile } => {
                self.identities.add_identity(profile);
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

impl LifecycleView for UserView {
    type Aggregate = User;

    fn create(event: CreateEnvelope<'_, Self::Aggregate>) -> Self {
        let UserCreated { name } = event.payload;
        Self {
            id: event.aggregate_id,
            name: name.clone(),
            identities: Default::default(),
        }
    }

    fn update(&mut self, event: UpdateEnvelope<'_, Self::Aggregate>) {
        match &event.payload {
            UserUpdated::IdentityAdded { profile } => {
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

impl View for IdentityView {
    type Aggregate = UserAggregate;
}

pub struct IdentityQuery<R>
where
    R: ViewRepository<IdentityView>,
{
    view_repository: Arc<R>,
}

impl<R> IdentityQuery<R>
where
    R: ViewRepository<IdentityView>,
{
    pub fn new(view_repository: Arc<R>) -> Self {
        Self { view_repository }
    }
}

#[async_trait]
impl<R> Query<UserAggregate> for IdentityQuery<R>
where
    R: ViewRepository<IdentityView>,
{
    async fn dispatch(&self, user_id: UserId, events: &[LifecycleEnvelope<User>]) {
        for event in events {
            if let LifecycleEvent::Updated(UserUpdated::IdentityAdded { profile }) = &event.payload
            {
                let identity_id = profile.identity().to_string();

                self.view_repository
                    .load_modify_update(
                        &identity_id,
                        |view| {
                            warn!("Identity already exists, reassigning to another user");
                            view.user_id = user_id;
                        },
                        || IdentityView { user_id },
                    )
                    .await
                    .unwrap();
            }
        }
    }
}
