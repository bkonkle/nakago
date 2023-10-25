use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Tag};

use crate::{
    config::CONFIG,
    domains::{episodes, profiles, shows, users},
};
use crate::{
    domains::{
        episodes,
        profiles::resolver::{ProfilesMutation, ProfilesQuery},
        shows::resolver::{ShowsMutation, ShowsQuery},
        users::resolver::{UsersMutation, UsersQuery},
    },
    utils::authz::OSO,
};

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, ProfilesQuery, ShowsQuery, EpisodesQuery);

/// The GraphQL top-level Mutation type
#[derive(MergedObject, Default)]
pub struct Mutation(
    UsersMutation,
    ProfilesMutation,
    ShowsMutation,
    EpisodesMutation,
);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Tag(GraphQLSchema)
pub const SCHEMA: Tag<GraphQLSchema> = Tag::new("GraphQLSchema");

/// Tag(GraphQLSchemaBuilder)
pub const SCHEMA_BUILDER: Tag<SchemaBuilder<Query, Mutation, EmptySubscription>> =
    Tag::new("GraphQLSchemaBuilder");

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
