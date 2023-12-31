# Summary

[CQRS and Event Sourcing using Rust](intro.md)
- [The patterns](theory.md)
  - [Domain driven design](theory_ddd.md)
  - [CQRS](theory_cqrs.md)
  - [Making changes to the application state](theory_updates.md)
  - [Queries](theory_queries.md)
  - [Event Sourcing](theory_event_sourcing.md)
- [Getting started](intro_getting_started.md)
  - [Add commands](intro_add_commands.md)
  - [Add domain events](intro_add_events.md)
  - [Add an error and service](intro_add_error.md)
  - [Add an aggregate](intro_add_aggregate.md)
- [Domain tests](test_add_first.md)
  - [Adding more complex logic](test_add_more.md)
- [Configuring a (test) application](demo_application.md)
  - [An event store](demo_event_store.md)
  - [A simple query](demo_simple_query.md)
  - [Putting everything together](demo_application_framework.md)
- [Building an application](application_building.md)
  - [Persisted event store](application_event_store.md)
  - [Queries with persisted views](application_persisted_views.md)
  - [Including metadata](application_metadata.md)
  - [Event upcasters](advanced_event_upcasters.md)
