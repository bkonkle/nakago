---
sidebar_position: 5
---

# Async-GraphQL

The GraphQL integration is built around a flexible Schema Builder approach that allows you to modify the in-progress schema to incrementally add things like DataLoaders and other context for your Resolvers. It uses an init function as a trigger to build the final schema, making it available to the rest of your application through type ID or Tag.

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

## Loading the Schema

Organization is up to you and it can depend on the needs of the project, but it can help to separate your application's startup into multiple phases - like a "load" and an "init" phase. In the load phase you can use helpers to register all of the necessary Providers with your Inject container, including ones that are built up incrementally in a "plugin-style" architecture. Your GraphQL schema may depend on the domains that you have enabled for a particular entry point, just as your config may differ depending on the services you have enabled for that entry point. You can use whatever convention you like, passing the Inject container around as needed.

For example, in a Users domain you might want to provide a Service and a DataLoader along with a Query and a Mutation that will use them, all focused on Users. You call the function "load" because it will be invoked during the load phase where all Providers are in the process of being registered. You separate this from an "init" function that you will use later on, once all of the dependencies are loaded. That load function might look like this:

```rust
use super::{loaders, mutation, query, service, Loader, Mutation, Query, Service};

pub async fn load(i: &Inject) -> nakago::Result<()> {
    i.provide::<Box<dyn Service>>(service::Provide::default()).await?;
    i.provide::<DataLoader<Loader>>(loaders::Provide::default()).await?;
    i.provide::<Query>(query::Provide::default()).await?;
    i.provide::<Mutation>(mutation::Provide::default()).await?;

    Ok(())
}
```

To collect all of the dependencies needed for a particular application, you might have a top-level `graphql.rs` module that contains higher-level load function that simply composes together the smaller individual loaders for each domain:

```rust
use super::{episodes, profiles, role_grants, shows, users};

pub async fn load(i: &Inject) -> nakago::Result<()> {
    users::schema::load(&i).await?;
    profiles::schema::load(&i).await?;
    role_grants::schema::load(&i).await?;
    shows::schema::load(&i).await?;
    episodes::schema::load(&i).await?;

    Ok(())
}
```

In your application's top-level `init.rs` file, you could then simply add use this top-level loader if you wanted to load all available domain Providers at the same time:

```rust
pub async fn app(config_path: Option<PathBuf>) -> nakago::Result<Inject> {
    let i = Inject::default();

    // ...

    graphql::load(&i).await?;

    // ...

    Ok(i)
}
```

## Initialization

Following the convention established in the previous example - for the init phase, you could provide a top-level SchemaBuilder that consumes the those dependencies so that they can be combined together in a Schema.

To do this, you would first build your schema's `MergedObject` approach that is standard in `async_graphql`, generating your top-level Query and Mutation types:

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

Finally, define an init function that constructs your top-level SchemaBuilder and injects it to the container so that it will be available for your application:

```rust
pub async fn init(i: &Inject) -> nakago::Result<()> {
    let config = i.get::<Config>().await?;

    let users_query = i.consume::<users::Query>().await?;
    let profiles_query = i.consume::<profiles::Query>().await?;

    let users_mutation = i.consume::<users::Mutation>().await?;
    let profiles_mutation = i.consume::<profiles::Mutation>().await?;

    let builder = Schema::build(
        Query(users_query, profiles_query),
        Mutation(users_mutation, profiles_mutation),
        EmptySubscription,
    )
    .data(config.clone());

    i.inject::<SchemaBuilder>(builder).await?;

    users::schema::init(&i).await?;
    profiles::schema::init(&i).await?;
    role_grants::schema::init(&i).await?;

    schema::Init::<Query, Mutation, EmptySubscription>::default()
        .init(&i)
        .await?;

    Ok(())
}
```

In your application's top-level `init.rs` file, you can then add this GraphQL Schema initializer:

```rust
pub async fn app(config_path: Option<PathBuf>) -> nakago::Result<Inject> {
    let i = Inject::default();

    // ...

    graphql::init(&i).await?;

    // ...

    Ok(i)
}
```
