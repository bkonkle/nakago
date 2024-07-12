use async_graphql::{self, EmptySubscription, MergedObject};
use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};
use nakago_async_graphql::schema;
use oso::Oso;

use crate::Config;

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

/// Initializes the GraphQL schema builder
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let config = i.get_type::<Config>().await?;
        let oso = i.get_type::<Oso>().await?;

        let users_query = i.consume(&users::QUERY).await?;
        let profiles_query = i.consume(&profiles::QUERY).await?;
        let shows_query = i.consume(&shows::QUERY).await?;
        let episodes_query = i.consume(&episodes::QUERY).await?;

        let users_mutation = i.consume(&users::MUTATION).await?;
        let profiles_mutation = i.consume(&profiles::MUTATION).await?;
        let shows_mutation = i.consume(&shows::MUTATION).await?;
        let episodes_mutation = i.consume(&episodes::MUTATION).await?;

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

        i.inject_type::<SchemaBuilder>(builder).await?;

        i.handle(users::schema::Init::default()).await?;
        i.handle(profiles::schema::Init::default()).await?;
        i.handle(role_grants::schema::Init::default()).await?;
        i.handle(shows::schema::Init::default()).await?;
        i.handle(episodes::schema::Init::default()).await?;

        i.handle(schema::Init::default()).await?;

        Ok(())
    }
}
