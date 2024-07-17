use async_graphql::{self, EmptySubscription, MergedObject};
use nakago::Inject;
use nakago_async_graphql::schema;
use oso::Oso;

use crate::config::Config;

use super::{episodes, profiles, role_grants, shows, users};

/// The GraphQL top-level Query type
#[derive(MergedObject)]
pub struct Query(users::Query, profiles::Query, shows::Query, episodes::Query);

/// The GraphQL top-level Mutation type
#[derive(MergedObject)]
pub struct Mutation(
    users::Mutation,
    profiles::Mutation,
    shows::Mutation,
    episodes::Mutation,
);

/// The application's top-level merged GraphQL schema
pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// The application's top-level GraphQL schema builder
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription>;

/// Loads the GraphQL schema builder dependencies
pub async fn load(i: &Inject) -> nakago::Result<()> {
    users::schema::load(&i).await?;
    profiles::schema::load(&i).await?;
    role_grants::schema::load(&i).await?;
    shows::schema::load(&i).await?;
    episodes::schema::load(&i).await?;

    Ok(())
}

/// Initializes the GraphQL schema builder
pub async fn init(i: &Inject) -> nakago::Result<()> {
    let config = i.get::<Config>().await?;
    let oso = i.get::<Oso>().await?;

    let users_query = i.consume::<users::Query>().await?;
    let profiles_query = i.consume::<profiles::Query>().await?;
    let shows_query = i.consume::<shows::Query>().await?;
    let episodes_query = i.consume::<episodes::Query>().await?;

    let users_mutation = i.consume::<users::Mutation>().await?;
    let profiles_mutation = i.consume::<profiles::Mutation>().await?;
    let shows_mutation = i.consume::<shows::Mutation>().await?;
    let episodes_mutation = i.consume::<episodes::Mutation>().await?;

    let builder = Schema::build(
        Query(users_query, profiles_query, shows_query, episodes_query),
        Mutation(
            users_mutation,
            profiles_mutation,
            shows_mutation,
            episodes_mutation,
        ),
        EmptySubscription,
    )
    .data(config.clone())
    .data((*oso).clone());

    i.inject::<SchemaBuilder>(builder).await?;

    users::schema::init(&i).await?;
    profiles::schema::init(&i).await?;
    role_grants::schema::init(&i).await?;
    shows::schema::init(&i).await?;
    episodes::schema::init(&i).await?;

    schema::Init::<Query, Mutation, EmptySubscription>::default()
        .init(&i)
        .await?;

    Ok(())
}
