---
sidebar_position: 4
---

# Examples

## Async-GraphQL

The Async-GraphQL example demonstrates an example Application architecture using Async-GraphQL, SeaORM, and Axum.

-   [Main](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/main.rs) is where the AxumApplication is initialized, using `pico_args` for light argument parsing.
-   [Axum Routes](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/routes.rs) are defined in `routes.rs`, along with an Inject Provider.
-   [Axum Handlers](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/handlers.rs) are defined in `handlers.rs`.
-   The [GraphQL Schema](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/graphql.rs) is initialized in `graphql.rs`.
-   The [InitApp](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/init.rs) Hook intializes dependencies for Startup.
-   [Domains](https://github.com/bkonkle/nakago/tree/feature/nakago-sea-orm/examples/async-graphql/src/domains) are defined in the `domains` directory, and handle Database Models, GraphQL Resolvers, supporting Services, and more.

This application will be slowly integrated into the framework itself, leaving a clean implementation of what is needed for your particular use cases rather than reinventing the wheel for each application.

### Notes

-   Copy `.envrc.example` to `.envrc` and edit to fill in live values.
-   `cargo make db-create-async-graphql`
-   `cargo make db-migrate-async-graphql`

-   _Coming soon:_ `cargo make db-create-cqrs-es`
-   _Coming soon:_ `cargo make db-migrate-cqrs-es`

-   `cargo make integration`
