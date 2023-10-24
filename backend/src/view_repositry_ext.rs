use cqrs_es::persist::ViewRepository;
use cqrs_es::{Aggregate, View};

trait ViewRepositoryExt<V, A>: ViewRepository<V, A>
where
    V: View<A>,
    A: Aggregate,
{
}
