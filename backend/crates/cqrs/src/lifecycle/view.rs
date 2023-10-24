use crate::lifecycle::LifecycleAggregate;

pub trait LifecycleView {
    type Aggregate: LifecycleAggregate;
}

pub enum LifecycleViewState<V: LifecycleView> {
    NotCreated,
    Created(V),
    Deleted,
}

// impl View<>
