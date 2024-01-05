---
sidebar_position: 5
---

# Async-GraphQL

The GraphQL integration is built around a flexible Schema Builder approach that allows you to modify the in-progress schema to incrementally add things like DataLoaders and other context for your Resolvers. It uses the Init lifecycle hook as a trigger to build the final schema, making it available to the rest of your application through type ID or Tag.

## Dependencies vs. Context

Async-GraphQL provides its own lightweight synchronous dependency injection system within their request Context, called "data". In typical Async-GraphQL applications, this is used similarly to Axum State to provide dependencies that each Resolver needs to do its job.

As we do with the Axum integration, for Nakago apps we separate the concept of Dependencies from the concept of Context, and instead use the Inject container to provide dependencies that remain the same across all requests. Data that varies from request to request should still be carried through the Context data, as usual.

## Resolvers

```rust
/// The Query segment for Users
#[derive(Default)]
pub struct UsersQuery {}

/// Queries for the User model
#[Object]
impl UsersQuery {
    /// Get the current User from the GraphQL context
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let user = ctx.data_unchecked::<Option<User>>();

        Ok(user.clone())
    }
}
```

## Loading

One strategy to keep things encapsulated is to use a domain-specific Load lifecycle hook to inject all of the Providers that are specific to a particular entity or area of concern - like Users or Profiles or other application-specific focuses.

For example, a Load hook for a Users domain might want to provide a Service and a DataLoader along with a Query and a Mutation that will use them, all focused on Users:

```rust
use super::{loaders, mutation, query, service, LOADER, MUTATION, SERVICE, QUERY};

#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide(&SERVICE, service::Provide::default()).await?;
        i.provide(&LOADER, loaders::Provide::default()).await?;
        i.provide(&QUERY, query::Provide::default()).await?;
        i.provide(&MUTATION, mutation::Provide::default()).await?;

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
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
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

In the Init phase, you can provide a top-level SchemaBuilder that other modules can optionally extend, culminating in a fully operation schema ready to execute GraphQL operations.

First, build your schema using the standard `async_graphql` approach, building MergedObjects for your top-level Query and Mutation types:

```rust
/// The GraphQL top-level Query type
#[derive(MergedObject)]
pub struct Query(users::Query, profiles::Query);

/// The GraphQL top-level Mutation type
#[derive(MergedObject)]
pub struct Mutation(
    users::Mutation,
    profiles::Mutation,
);

/// The application's top-level merged GraphQL schema
pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// The application's top-level GraphQL schema builder
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription>;
```

Then, provide tags to represent each type in the Inject container:

```rust
/// Tag(graphql::Schema)
pub const SCHEMA: Tag<Schema> = Tag::new("graphql::Schema");

/// Tag(graphql::SchemaBuilder)
pub const SCHEMA_BUILDER: Tag<SchemaBuilder> = Tag::new("graphql::SchemaBuilder");
```

Finally, define an Init hook that constructs your top-level SchemaBuilder and injects it to the container so that it will be available for any Init hooks that want to add context:

```rust
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let users_query = i.consume(&users::QUERY).await?;
        let profiles_query = i.consume(&profiles::QUERY).await?;

        let users_mutation = i.consume(&users::MUTATION).await?;
        let profiles_mutation = i.consume(&profiles::MUTATION).await?;

        let builder = Schema::build(
            Query(users_query, profiles_query),
            Mutation(users_mutation, profiles_mutation),
            EmptySubscription,
        );

        i.inject(&SCHEMA_BUILDER, builder).await?;

        i.handle(users::schema::Init::default()).await?;
        i.handle(profiles::schema::Init::default()).await?;

        i.handle(
            schema::Init::default()
                .with_builder_tag(&SCHEMA_BUILDER)
                .with_schema_tag(&SCHEMA),
        )
        .await?;

        Ok(())
    }
}
```

In your application's top-level `init.rs` file, you can then add this top-level GraphQL Init hook to the list of hooks that are run in response to the Init lifecycle event:

```rust
pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    // ...

    app.on(&EventType::Init, graphql::Init::default());

    // ...

    Ok(app)
}
```
