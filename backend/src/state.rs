use crate::domain::ticket::{Ticket, TicketView};
use crate::domain::user::{IdentityQuery, IdentityView, User, UserServices, UserView};
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
    pub ticket_view_repository: Arc<MyViewRepository<TicketView, Ticket>>,
    pub ticket_cqrs: Arc<MyCqrsFramework<Ticket>>,

    pub user_view_repository: Arc<MyViewRepository<UserView, User>>,
    pub user_identity_view_repository: Arc<MyViewRepository<IdentityView, User>>,
    pub user_cqrs: Arc<MyCqrsFramework<User>>,
}

pub async fn new_application_state() -> ApplicationState {
    let ticket_view_repository = Arc::new(MyViewRepository::<TicketView, Ticket>::new());
    let ticket_view_query =
        MyGenericQuery::<TicketView, Ticket>::new(ticket_view_repository.clone());

    let ticket_cqrs = CqrsFramework::new(
        MemStore::<Ticket>::default(),
        vec![Box::new(ticket_view_query)],
        (),
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
        ticket_view_repository,
        ticket_cqrs,

        user_view_repository,
        user_identity_view_repository,
        user_cqrs,
    }
}
