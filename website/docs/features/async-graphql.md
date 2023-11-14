---
sidebar_position: 5
---

# Async-GraphQL

The GraphQL integration is built around a flexible Schema Builder approach that allows you to modify the in-progress schema to incrementally add things like DataLoaders and other context for your Resolvers. It uses the Init lifecycle hook as a trigger to build the final schema, making it available to the rest of your application through type ID or Tag.

## Loading

One strategy to keep things encapsulated is to use the Load lifecycle hook to inject providers that are specific to your GraphQL schema - like the Services and DataLoaders that your Resolvers will interact with.

For example, a Load hook for a Users domain might want to provide a Users Service and a Users DataLoader, like this:

```rust
use super::{
    loaders::{self, LOADER},
    service::{self, SERVICE},
};

#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        i.provide(&SERVICE, service::Provide::default()).await?;
        i.provide(&LOADER, loaders::Provide::default()).await?;

        Ok(())
    }
}
```

To collect all of the dependencies needed for a particular application, you might have a top-level `graphql.rs` module that contains an Init hook that simply composes together the smaller individual Init hooks for each domain:

```rust
use super::{episodes, profiles, role_grants, shows, users};

#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        i.provide(&SCHEMA_BUILDER, Provide::default()).await?;

        i.handle(users::schema::Load::default()).await?;
        i.handle(profiles::schema::Load::default()).await?;
        i.handle(role_grants::schema::Load::default()).await?;
        i.handle(shows::schema::Load::default()).await?;
        i.handle(episodes::schema::Load::default()).await?;

        Ok(())
    }
}
```

In your application's top-level `init.rs` file, you could then simply add this top-level GraphQL Load hook to the list of hooks that are run in response to the Load lifecycle event:

```rust
pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    // ...

    app.on(&EventType::Load, graphql::Load::default());

    // ...

    Ok(app)
}
```

## Initialization
