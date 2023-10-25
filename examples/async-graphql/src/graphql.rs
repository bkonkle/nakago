use async_graphql::{self, EmptySubscription, MergedObject, SchemaBuilder};
use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Tag};

use crate::{
    config::CONFIG,
    domains::{episodes, profiles, shows, users},
    utils::authz::OSO,
};

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(users::Query, profiles::Query, shows::Query, episodes::Query);

/// The GraphQL top-level Mutation type
#[derive(MergedObject, Default)]
pub struct Mutation(
    users::Mutation,
    profiles::Mutation,
    shows::Mutation,
    episodes::Mutation,
);

/// The application's top-level merged GraphQL schema
pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// Tag(GraphQLSchema)
pub const SCHEMA: Tag<Schema> = Tag::new("GraphQLSchema");

/// Tag(SchemaBuilder)
pub const SCHEMA_BUILDER: Tag<SchemaBuilder<Query, Mutation, EmptySubscription>> =
    Tag::new("SchemaBuilder");

/// Initializes the GraphQL schema builder
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let config = i.get(&CONFIG).await?;
        let oso = i.get(&OSO).await?;

        i.modify(&SCHEMA_BUILDER, |builder| {
            Ok(builder.data(config.clone()).data((*oso).clone()))
        })
        .await?;

        i.handle(users::schema::Init::default()).await?;
        i.handle(profiles::schema::Init::default()).await?;
        i.handle(shows::schema::Init::default()).await?;
        i.handle(episodes::schema::Init::default()).await?;

        Ok(())
    }
}
