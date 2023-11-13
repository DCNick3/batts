use crate::auth::CookieAuthority;
use crate::config::TelegramSecret;
use crate::domain::group::{GroupAggregate, GroupView, UserGroupsQuery, UserGroupsView};
use crate::domain::ticket::{
    TicketAggregate, TicketListingKind, TicketListingQuery, TicketListingView, TicketServices,
    TicketView,
};
use crate::domain::user::{IdentityQuery, IdentityView, UserAggregate, UserServices, UserView};
use crate::elasticsearch_view_repository::ElasticViewRepository;
use crate::routes::UploadState;
use cqrs_es::lifecycle::{
    LifecycleAggregate, LifecycleAggregateState, LifecycleQuery, LifecycleView, LifecycleViewState,
};
use cqrs_es::mem_store::MemStore;
use cqrs_es::{Aggregate, CqrsFramework, Query, View};
use elasticsearch::Elasticsearch;
use std::collections::HashSet;
use std::default::Default;
use std::sync::Arc;
use tracing::{info, warn};

type MyCqrsFramework<A> = CqrsFramework<A, MemStore<A>>;

type MyViewRepository<V> = ElasticViewRepository<V>;
// type MyGenericQuery<V> = GenericQuery<MyViewRepository<V>, V>;

type MyLifecycleViewRepository<V> = ElasticViewRepository<LifecycleViewState<V>>;
// type MyLifecycleQuery<V> = LifecycleQuery<MyLifecycleViewRepository<V>, V>;

#[derive(Clone)]
pub struct ApplicationState {
    pub cookie_authority: CookieAuthority,
    pub telegram_login_secret: Option<TelegramSecret>,

    pub upload: Option<UploadState>,

    pub group_view_repository: Arc<MyLifecycleViewRepository<GroupView>>,
    pub user_groups_view_repository: Arc<MyViewRepository<UserGroupsView>>,
    pub group_cqrs: Arc<MyCqrsFramework<GroupAggregate>>,

    pub ticket_view_repository: Arc<MyLifecycleViewRepository<TicketView>>,
    pub ticket_owner_listing_view_repository: Arc<MyViewRepository<TicketListingView>>,
    pub ticket_assignee_listing_view_repository: Arc<MyViewRepository<TicketListingView>>,
    pub ticket_destination_listing_view_repository: Arc<MyViewRepository<TicketListingView>>,
    pub ticket_cqrs: Arc<MyCqrsFramework<TicketAggregate>>,

    pub user_view_repository: Arc<MyLifecycleViewRepository<UserView>>,
    pub user_identity_view_repository: Arc<MyViewRepository<IdentityView>>,
    pub user_cqrs: Arc<MyCqrsFramework<UserAggregate>>,
}

struct CqrsBuilder {
    elastic: Elasticsearch,
    elastic_index_names: HashSet<String>,
}

impl CqrsBuilder {
    // TODO: pass the event store
    fn new(elastic: Elasticsearch) -> Self {
        Self {
            elastic,
            elastic_index_names: HashSet::new(),
        }
    }

    fn aggregate<A: Aggregate>(&mut self, _name: &str) -> AggregateBuilder<A> {
        AggregateBuilder {
            cqrs: self,
            queries: Vec::new(),
        }
    }

    async fn finalize(self) {
        // wipe the elasticsearch database
        // TODO: remove this when we are able to handle persistence
        for index in &self.elastic_index_names {
            match crate::elasticsearch_view_repository::try_delete_index(
                self.elastic.transport(),
                index,
            )
            .await
            {
                Ok(true) => {
                    warn!("Deleted index `{}`", index);
                }
                Ok(false) => {}
                Err(e) => {
                    panic!("Failed deleting index `{}`: {:?}", index, e);
                }
            }
        }

        for index in &self.elastic_index_names {
            match crate::elasticsearch_view_repository::ensure_index_exists(
                self.elastic.transport(),
                index,
            )
            .await
            {
                Ok(true) => {
                    info!("Created index `{}`", index);
                }
                Ok(false) => {}
                Err(e) => {
                    panic!("Failed to create index `{}`: {:?}", index, e);
                }
            }
        }
    }
}

struct AggregateBuilder<'a, A: Aggregate> {
    cqrs: &'a mut CqrsBuilder,
    queries: Vec<Box<dyn Query<A>>>,
}

impl<'a, A: Aggregate> AggregateBuilder<'a, A> {
    fn view_repository<
        V: View,
        Q: Query<A> + 'static,
        FnQ: FnOnce(Arc<MyViewRepository<V>>) -> Q,
    >(
        &mut self,
        name: &str,
        f: FnQ,
    ) -> Arc<MyViewRepository<V>> {
        if !self.cqrs.elastic_index_names.insert(name.to_string()) {
            panic!("An index named `{}` already exists", name)
        }

        let view_repository = MyViewRepository::new(self.cqrs.elastic.clone(), name);
        let view_repository = Arc::new(view_repository);

        self.queries.push(Box::new(f(view_repository.clone())));

        view_repository
    }

    fn build(self, services: A::Services) -> Arc<MyCqrsFramework<A>> {
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
        name: &str,
    ) -> Arc<MyLifecycleViewRepository<V>> {
        self.view_repository(name, |repo| LifecycleQuery::new(repo))
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

    let elastic = elasticsearch::http::transport::TransportBuilder::new(
        elasticsearch::http::transport::SingleNodeConnectionPool::new(
            config.storage.elasticsearch.endpoint.clone(),
        ),
    )
    .auth(elasticsearch::auth::Credentials::Basic(
        config.storage.elasticsearch.user.clone(),
        config.storage.elasticsearch.password.clone(),
    ))
    .build()
    .expect("Failed to build elasticsearch transport");
    let elastic = Elasticsearch::new(elastic);

    let mut builder = CqrsBuilder::new(elastic);

    let mut groups_builder = builder.aggregate("groups");

    let group_view_repository = groups_builder.lifecycle_view_repository("groups");
    let user_groups_view_repository =
        groups_builder.view_repository("groups-user", UserGroupsQuery::new);

    let group_cqrs = groups_builder.build(());

    let mut tickets_builder = builder.aggregate("tickets");
    let ticket_view_repository = tickets_builder.lifecycle_view_repository("tickets");

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

    let user_view_repository = user_builder.lifecycle_view_repository("users");
    let user_identity_view_repository =
        user_builder.view_repository("users-identity", IdentityQuery::new);

    let user_cqrs = user_builder.build(UserServices {
        user_identity_view_repository: user_identity_view_repository.clone(),
    });

    builder.finalize().await;

    ApplicationState {
        cookie_authority: authority,
        telegram_login_secret: config.auth.telegram_secret.clone(),
        upload,

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
