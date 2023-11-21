use crate::auth::CookieAuthority;
use crate::config::TelegramSecret;
use crate::domain::group::{Group, GroupView, UserGroupsQuery, UserGroupsView};
use crate::domain::ticket::{
    Ticket, TicketListingKind, TicketListingQuery, TicketListingView, TicketServices, TicketView,
};
use crate::domain::user::{IdentityQuery, IdentityView, User, UserServices, UserView};
use crate::meilisearch_view_repository::MeilisearchViewRepository;
use crate::routes::UploadState;
use cqrs_es::lifecycle::{
    LifecycleAggregate, LifecycleAggregateState, LifecycleQuery, LifecycleView, LifecycleViewState,
};
use cqrs_es::mem_store::MemStore;
use cqrs_es::{Aggregate, CqrsFramework, Query, View};
use meilisearch_sdk::Index;
use std::collections::HashSet;
use std::default::Default;
use std::sync::Arc;
use tracing::{info, warn};

type MyEventStore<A> = MemStore<A>;
type MyCqrsFramework<A> =
    CqrsFramework<LifecycleAggregateState<A>, MyEventStore<LifecycleAggregateState<A>>>;

type MyViewRepository<V> = MeilisearchViewRepository<V>;
// type MyGenericQuery<V> = GenericQuery<MyViewRepository<V>, V>;

type MyLifecycleViewRepository<V> = MeilisearchViewRepository<LifecycleViewState<V>>;
// type MyLifecycleQuery<V> = LifecycleQuery<MyLifecycleViewRepository<V>, V>;

#[derive(Clone)]
pub struct ApplicationState {
    pub cookie_authority: CookieAuthority,
    pub telegram_login_secret: Option<TelegramSecret>,

    pub cqrs: CqrsState,
    pub search: SearchState,
    pub upload: Option<UploadState>,
}

#[derive(Clone)]
pub struct SearchState {
    pub meilisearch: meilisearch_sdk::Client,

    pub user_index: Index,
    pub group_index: Index,
    pub ticket_index: Index,
}

impl SearchState {
    pub async fn ensure_settings(&self) {
        let mut tasks = Vec::new();

        tasks.push(
            self.ticket_index
                .set_settings(&meilisearch_sdk::Settings {
                    searchable_attributes: Some(
                        // TODO: search by messages? maybe in a different index?
                        // honestly, we shouldn't be using meilisearch to store our data...
                        ["title"].into_iter().map(ToString::to_string).collect(),
                    ),
                    filterable_attributes: Some(
                        ["lifecycle_state"]
                            .into_iter()
                            .map(ToString::to_string)
                            .collect(),
                    ),
                    ..Default::default()
                })
                .await
                .expect("Failed to set ticket index settings"),
        );

        tasks.push(
            self.group_index
                .set_settings(&meilisearch_sdk::Settings {
                    searchable_attributes: Some(
                        ["title"].into_iter().map(ToString::to_string).collect(),
                    ),
                    filterable_attributes: Some(
                        ["lifecycle_state"]
                            .into_iter()
                            .map(ToString::to_string)
                            .collect(),
                    ),
                    ..Default::default()
                })
                .await
                .expect("Failed to set group index settings"),
        );

        tasks.push(
            self.user_index
                .set_settings(&meilisearch_sdk::Settings {
                    searchable_attributes: Some(
                        [
                            "name",
                            "identities.telegram.username",
                            "identities.university.email",
                        ]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                    ),
                    filterable_attributes: Some(
                        ["lifecycle_state"]
                            .into_iter()
                            .map(ToString::to_string)
                            .collect(),
                    ),
                    ..Default::default()
                })
                .await
                .expect("Failed to set user index settings"),
        );

        info!("Waiting for settings tasks to complete...");
        for task in tasks.drain(..) {
            task.wait_for_completion(&self.meilisearch, None, None)
                .await
                .expect("Failed to wait for settings task");
        }
    }
}

#[derive(Clone)]
pub struct CqrsState {
    pub group_view_repository: Arc<MyLifecycleViewRepository<GroupView>>,
    pub user_groups_view_repository: Arc<MyViewRepository<UserGroupsView>>,
    pub group_cqrs: Arc<MyCqrsFramework<Group>>,

    pub ticket_view_repository: Arc<MyLifecycleViewRepository<TicketView>>,
    pub ticket_owner_listing_view_repository: Arc<MyViewRepository<TicketListingView>>,
    pub ticket_assignee_listing_view_repository: Arc<MyViewRepository<TicketListingView>>,
    pub ticket_destination_listing_view_repository: Arc<MyViewRepository<TicketListingView>>,
    pub ticket_cqrs: Arc<MyCqrsFramework<Ticket>>,

    pub user_view_repository: Arc<MyLifecycleViewRepository<UserView>>,
    pub user_identity_view_repository: Arc<MyViewRepository<IdentityView>>,
    pub user_cqrs: Arc<MyCqrsFramework<User>>,
}

pub trait BattsAggregate: LifecycleAggregate {
    fn get_cqrs_state(state: &CqrsState) -> &Arc<MyCqrsFramework<Self>>;
}

pub trait BattsView: LifecycleView
where
    <Self as LifecycleView>::Aggregate: BattsAggregate,
{
    fn get_view_repository(state: &CqrsState) -> &Arc<MyLifecycleViewRepository<Self>>;
}

impl BattsAggregate for User {
    fn get_cqrs_state(state: &CqrsState) -> &Arc<MyCqrsFramework<Self>> {
        &state.user_cqrs
    }
}

impl BattsView for UserView {
    fn get_view_repository(state: &CqrsState) -> &Arc<MyLifecycleViewRepository<Self>> {
        &state.user_view_repository
    }
}

impl BattsAggregate for Group {
    fn get_cqrs_state(state: &CqrsState) -> &Arc<MyCqrsFramework<Self>> {
        &state.group_cqrs
    }
}

impl BattsView for GroupView {
    fn get_view_repository(state: &CqrsState) -> &Arc<MyLifecycleViewRepository<Self>> {
        &state.group_view_repository
    }
}

impl BattsAggregate for Ticket {
    fn get_cqrs_state(state: &CqrsState) -> &Arc<MyCqrsFramework<Self>> {
        &state.ticket_cqrs
    }
}

impl BattsView for TicketView {
    fn get_view_repository(state: &CqrsState) -> &Arc<MyLifecycleViewRepository<Self>> {
        &state.ticket_view_repository
    }
}

struct CqrsBuilder {
    meilisearch: meilisearch_sdk::Client,
    index_names: HashSet<String>,
}

impl CqrsBuilder {
    // TODO: pass the event store
    fn new(meilisearch: meilisearch_sdk::Client) -> Self {
        Self {
            meilisearch,
            index_names: HashSet::new(),
        }
    }

    fn aggregate<A: Aggregate>(&mut self, _name: &str) -> AggregateBuilder<A> {
        AggregateBuilder {
            cqrs: self,
            queries: Vec::new(),
        }
    }

    async fn finalize(self) {
        let mut tasks = Vec::new();

        // wipe the elasticsearch database
        // TODO: remove this when we are able to handle persistence
        for index in &self.index_names {
            match self.meilisearch.delete_index(index).await {
                Ok(task) => {
                    tasks.push(task);
                    warn!("Deleted index `{}`", index);
                }
                Err(meilisearch_sdk::Error::Meilisearch(meilisearch_sdk::MeilisearchError {
                    error_code: meilisearch_sdk::ErrorCode::IndexNotFound,
                    ..
                })) => {}
                Err(e) => {
                    panic!("Failed deleting index `{}`: {:?}", index, e);
                }
            }
        }

        info!("Waiting for deletion tasks to complete...");
        for task in tasks.drain(..) {
            task.wait_for_completion(&self.meilisearch, None, None)
                .await
                .expect("Failed to wait for deletion task");
        }

        for index in &self.index_names {
            match self.meilisearch.create_index(index, Some("_view_id")).await {
                Ok(task) => {
                    tasks.push(task);
                    info!("Created index `{}`", index);
                }
                Err(meilisearch_sdk::Error::Meilisearch(meilisearch_sdk::MeilisearchError {
                    error_code: meilisearch_sdk::ErrorCode::IndexAlreadyExists,
                    ..
                })) => {}
                Err(e) => {
                    panic!("Failed to create index `{}`: {:?}", index, e);
                }
            }
        }

        info!("Waiting for creation tasks to complete...");
        for task in tasks.drain(..) {
            task.wait_for_completion(&self.meilisearch, None, None)
                .await
                .expect("Failed to wait for creation task");
        }
    }
}

struct AggregateBuilder<'a, A: Aggregate> {
    cqrs: &'a mut CqrsBuilder,
    queries: Vec<Box<dyn Query<A>>>,
}

impl<'a, A: Aggregate> AggregateBuilder<'a, A> {
    fn view_repository_from_index<
        V: View,
        Q: Query<A> + 'static,
        FnQ: FnOnce(Arc<MyViewRepository<V>>) -> Q,
    >(
        &mut self,
        index: Index,
        f: FnQ,
    ) -> Arc<MyViewRepository<V>> {
        if !self.cqrs.index_names.insert(index.uid.clone()) {
            panic!("An index named `{}` already exists", index.uid)
        }

        let view_repository = MyViewRepository::new(index.clone());
        let view_repository = Arc::new(view_repository);

        self.queries.push(Box::new(f(view_repository.clone())));

        view_repository
    }

    fn view_repository<
        V: View,
        Q: Query<A> + 'static,
        FnQ: FnOnce(Arc<MyViewRepository<V>>) -> Q,
    >(
        &mut self,
        name: &str,
        f: FnQ,
    ) -> Arc<MyViewRepository<V>> {
        let index = Index::new(name, self.cqrs.meilisearch.clone());
        self.view_repository_from_index(index, f)
    }

    fn build(self, services: A::Services) -> Arc<CqrsFramework<A, MyEventStore<A>>> {
        Arc::new(CqrsFramework::new(
            // TODO: use an actual event store
            MemStore::default(),
            self.queries,
            services,
        ))
    }
}

impl<'a, A: LifecycleAggregate> AggregateBuilder<'a, LifecycleAggregateState<A>> {
    fn lifecycle_view_repository<V: LifecycleView<Aggregate = A> + 'static>(
        &mut self,
        index: Index,
    ) -> Arc<MyLifecycleViewRepository<V>> {
        self.view_repository_from_index(index, |repo| LifecycleQuery::new(repo))
    }
}

async fn search_state(config: &crate::config::Config) -> SearchState {
    let meilisearch = meilisearch_sdk::Client::new(
        config.storage.meilisearch.endpoint.to_string(),
        Some(&config.storage.meilisearch.api_key),
    );

    let group_index = meilisearch.index("groups");
    let ticket_index = meilisearch.index("tickets");
    let user_index = meilisearch.index("users");

    SearchState {
        meilisearch,
        user_index,
        group_index,
        ticket_index,
    }
}

async fn cqrs_state(search_state: &SearchState) -> CqrsState {
    let mut builder = CqrsBuilder::new(search_state.meilisearch.clone());

    let mut groups_builder = builder.aggregate("groups");

    let group_view_repository =
        groups_builder.lifecycle_view_repository(search_state.group_index.clone());
    let user_groups_view_repository =
        groups_builder.view_repository("groups-user", UserGroupsQuery::new);

    let group_cqrs = groups_builder.build(());

    let mut tickets_builder = builder.aggregate("tickets");
    let ticket_view_repository =
        tickets_builder.lifecycle_view_repository(search_state.ticket_index.clone());

    fn make_ticket_listing(
        kind: TicketListingKind,
    ) -> impl FnOnce(
        Arc<MyViewRepository<TicketListingView>>,
    ) -> TicketListingQuery<MyViewRepository<TicketListingView>> {
        move |repo| TicketListingQuery::new(repo, kind)
    }
    // TODO: a lot of those become redundant if we use elastic's filter capabilities
    let ticket_owner_listing_view_repository = tickets_builder.view_repository(
        "tickets-owner-listing",
        make_ticket_listing(TicketListingKind::Owned),
    );
    let ticket_assignee_listing_view_repository = tickets_builder.view_repository(
        "tickets-assignee-listing",
        make_ticket_listing(TicketListingKind::Assigned),
    );
    let ticket_destination_listing_view_repository = tickets_builder.view_repository(
        "tickets-destination-listing",
        make_ticket_listing(TicketListingKind::Destination),
    );

    let ticket_cqrs = tickets_builder.build(TicketServices {
        group_view_repository: group_view_repository.clone(),
    });

    let mut user_builder = builder.aggregate("users");

    let user_view_repository =
        user_builder.lifecycle_view_repository(search_state.user_index.clone());
    let user_identity_view_repository =
        user_builder.view_repository("users-identity", IdentityQuery::new);

    let user_cqrs = user_builder.build(UserServices {
        user_identity_view_repository: user_identity_view_repository.clone(),
    });

    builder.finalize().await;

    CqrsState {
        ticket_view_repository,
        ticket_owner_listing_view_repository,
        ticket_assignee_listing_view_repository,
        ticket_destination_listing_view_repository,
        ticket_cqrs,

        group_view_repository,
        user_groups_view_repository,
        group_cqrs,

        user_view_repository,
        user_identity_view_repository,
        user_cqrs,
    }
}

pub async fn new_application_state(config: &crate::config::Config) -> ApplicationState {
    let authority = CookieAuthority::new(
        "session",
        ed25519_dalek::Keypair::from_bytes(&[
            // TODO: replace this hard-coded key with something more secure
            0x5c, 0x6a, 0xc5, 0xf2, 0xb8, 0x12, 0xf1, 0x9d, 0x7e, 0x70, 0xd1, 0xe4, 0x9a, 0x28,
            0x20, 0xa6, 0x5b, 0xba, 0xb8, 0x9a, 0xa3, 0x76, 0x0d, 0xb0, 0x80, 0x53, 0xe4, 0x3d,
            0x7a, 0x5d, 0x27, 0x08, 0x3a, 0xb6, 0xf8, 0x28, 0xf2, 0x69, 0x04, 0x61, 0xd7, 0x05,
            0xdb, 0x89, 0x1d, 0x0d, 0xef, 0x94, 0x6e, 0xdd, 0xc2, 0x44, 0xf2, 0x92, 0xa3, 0x67,
            0x71, 0x80, 0x31, 0xe5, 0xb2, 0xcb, 0x8f, 0xc0,
        ])
        .unwrap(),
        chrono::Duration::from_std(config.auth.token_duration).unwrap(),
    );

    let upload = config.upload.as_ref().map(|upload| {
        let s3 = &upload.s3;

        let region = s3::Region::Custom {
            region: "us-east1".to_owned(),
            endpoint: s3.endpoint.to_string(),
        };
        let credentials = s3::creds::Credentials {
            access_key: Some(s3.access_key.clone()),
            secret_key: Some(s3.secret_key.clone()),
            security_token: None,
            session_token: None,
            expiration: None,
        };
        let bucket = s3::Bucket::new(&s3.bucket, region, credentials)
            .expect("Failed to create S3 bucket")
            .with_path_style();

        UploadState {
            bucket,
            policy: upload.policy.clone(),
        }
    });

    let search = search_state(config).await;
    let cqrs = cqrs_state(&search).await;

    search.ensure_settings().await;

    ApplicationState {
        cookie_authority: authority,
        telegram_login_secret: config.auth.telegram_secret.clone(),
        cqrs,
        search,
        upload,
    }
}
