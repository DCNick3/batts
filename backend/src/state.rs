use crate::auth::CookieAuthority;
use crate::domain::group::{Group, GroupView};
use crate::domain::ticket::{
    Ticket, TicketListingKind, TicketListingQuery, TicketListingView, TicketServices, TicketView,
};
use crate::domain::user::{IdentityQuery, IdentityView, User, UserServices, UserView};
use crate::login::TelegramSecret;
use crate::memory_view_repository::MemViewRepository;
use cqrs_es::mem_store::MemStore;
use cqrs_es::persist::GenericQuery;
use cqrs_es::CqrsFramework;
use std::sync::Arc;

type MyCqrsFramework<A> = CqrsFramework<A, MemStore<A>>;
type MyViewRepository<V, A> = MemViewRepository<V, A>;
type MyGenericQuery<V, A> = GenericQuery<MyViewRepository<V, A>, V, A>;

#[derive(Clone)]
pub struct ApplicationState {
    pub cookie_authority: CookieAuthority,
    pub telegram_login_secret: Option<TelegramSecret>,

    pub group_view_repository: Arc<MyViewRepository<GroupView, Group>>,
    pub group_cqrs: Arc<MyCqrsFramework<Group>>,

    pub ticket_view_repository: Arc<MyViewRepository<TicketView, Ticket>>,
    pub ticket_owner_listing_view_repository: Arc<MyViewRepository<TicketListingView, Ticket>>,
    pub ticket_assignee_listing_view_repository: Arc<MyViewRepository<TicketListingView, Ticket>>,
    pub ticket_destination_listing_view_repository:
        Arc<MyViewRepository<TicketListingView, Ticket>>,
    pub ticket_cqrs: Arc<MyCqrsFramework<Ticket>>,

    pub user_view_repository: Arc<MyViewRepository<UserView, User>>,
    pub user_identity_view_repository: Arc<MyViewRepository<IdentityView, User>>,
    pub user_cqrs: Arc<MyCqrsFramework<User>>,
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

    let group_view_repository = Arc::new(MyViewRepository::<GroupView, Group>::new());
    let group_view_query = MyGenericQuery::<GroupView, Group>::new(group_view_repository.clone());
    let group_cqrs = CqrsFramework::new(
        MemStore::<Group>::default(),
        vec![Box::new(group_view_query)],
        (),
    );
    let group_cqrs = Arc::new(group_cqrs);

    let ticket_view_repository = Arc::new(MyViewRepository::<TicketView, Ticket>::new());
    let ticket_view_query =
        MyGenericQuery::<TicketView, Ticket>::new(ticket_view_repository.clone());

    let ticket_owner_listing_view_repository =
        Arc::new(MyViewRepository::<TicketListingView, Ticket>::new());
    let ticket_owner_listing_view_query = TicketListingQuery::new(
        ticket_owner_listing_view_repository.clone(),
        TicketListingKind::Owned,
    );
    let ticket_assignee_listing_view_repository =
        Arc::new(MyViewRepository::<TicketListingView, Ticket>::new());
    let ticket_assignee_listing_view_query = TicketListingQuery::new(
        ticket_assignee_listing_view_repository.clone(),
        TicketListingKind::Assigned,
    );
    let ticket_destination_listing_view_repository =
        Arc::new(MyViewRepository::<TicketListingView, Ticket>::new());
    let ticket_destination_listing_view_query = TicketListingQuery::new(
        ticket_destination_listing_view_repository.clone(),
        TicketListingKind::Destination,
    );

    let ticket_cqrs = CqrsFramework::new(
        MemStore::<Ticket>::default(),
        vec![
            Box::new(ticket_view_query),
            Box::new(ticket_owner_listing_view_query),
            Box::new(ticket_assignee_listing_view_query),
            Box::new(ticket_destination_listing_view_query),
        ],
        TicketServices {
            group_view_repository: group_view_repository.clone(),
        },
    );
    let ticket_cqrs = Arc::new(ticket_cqrs);

    let user_view_repository = Arc::new(MyViewRepository::<UserView, User>::new());
    let user_view_query = MyGenericQuery::<UserView, User>::new(user_view_repository.clone());

    let user_identity_view_repository = Arc::new(MyViewRepository::<IdentityView, User>::new());
    let user_identity_view_query = IdentityQuery::new(user_identity_view_repository.clone());

    let user_cqrs = CqrsFramework::new(
        MemStore::<User>::default(),
        vec![
            Box::new(user_view_query),
            Box::new(user_identity_view_query),
        ],
        UserServices {
            user_identity_view_repository: user_identity_view_repository.clone(),
        },
    );
    let user_cqrs = Arc::new(user_cqrs);

    ApplicationState {
        cookie_authority: authority,
        telegram_login_secret: config.auth.telegram_secret.clone(),

        ticket_view_repository,
        ticket_owner_listing_view_repository,
        ticket_assignee_listing_view_repository,
        ticket_destination_listing_view_repository,
        ticket_cqrs,

        group_view_repository,
        group_cqrs,

        user_view_repository,
        user_identity_view_repository,
        user_cqrs,
    }
}
