use cqrs_es::persist::ViewRepository;
use cqrs_es::View;

trait ViewRepositoryExt<V>: ViewRepository<V>
where
    V: View,
{
}
