---
sidebar_position: 4
---

# Examples

## Simple

The Simple example is a very simple Axum HTTP service using JWT with JWKS for authentication, with a single route that returns a JSON response and an integration test that calls it.

## Async-GraphQL

The Async-GraphQL example demonstrates an example Application architecture using Async-GraphQL, SeaORM, and Axum.

- [Main](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/main.rs) is where the AxumApplication is initialized, using `pico_args` for light argument parsing.
- [Axum Routes](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/http/routes.rs) are defined in `routes.rs`, along with an Inject Provider.
- [Axum Handlers](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/http/handlers.rs) are defined in `handlers.rs`.
- The [GraphQL Schema](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/graphql.rs) is initialized in `graphql.rs`.
- The [init::app()](https://github.com/bkonkle/nakago/blob/feature/nakago-sea-orm/examples/async-graphql/src/init.rs) function intializes dependencies.
- [Domains](https://github.com/bkonkle/nakago/tree/feature/nakago-sea-orm/examples/async-graphql/src/domains) are defined in the `domains` directory, and handle Database Models, GraphQL Resolvers, supporting Services, and more.

This application will be slowly integrated into the framework itself, leaving a clean implementation of what is needed for your particular use cases rather than reinventing the wheel for each application.
