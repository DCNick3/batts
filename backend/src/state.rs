use crate::domain::ticket::{Ticket, TicketView};
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
    // pub account_query: Arc<PostgresViewRepository<BankAccountView, BankAccount>>,
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

    ApplicationState {
        ticket_view_repository,
        ticket_cqrs,
    }
}
