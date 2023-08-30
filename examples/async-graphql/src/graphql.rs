use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Tag};

use crate::{
    config::AppConfig,
    domains::{
        episodes::schema::InitGraphQLEpisodes, profiles::schema::InitGraphQLProfiles,
        shows::schema::InitGraphQLShows, users::schema::InitGraphQLUsers,
    },
};
use crate::{
    domains::{
        episodes::resolver::{EpisodesMutation, EpisodesQuery},
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
pub const GRAPHQL_SCHEMA: Tag<GraphQLSchema> = Tag::new("GraphQLSchema");

/// Tag(GraphQLSchemaBuilder)
pub const GRAPHQL_SCHEMA_BUILDER: Tag<SchemaBuilder<Query, Mutation, EmptySubscription>> =
    Tag::new("GraphQLSchemaBuilder");

/// Initializes the GraphQL schema builder
#[derive(Default)]
pub struct InitGraphQL {}

#[async_trait]
impl Hook for InitGraphQL {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let config = i.get_type::<AppConfig>().await?;
        let oso = i.get(&OSO).await?;

        let builder = i.consume(&GRAPHQL_SCHEMA_BUILDER).await?;

        i.inject(
            &GRAPHQL_SCHEMA_BUILDER,
            builder.data(config.clone()).data((*oso).clone()),
        )
        .await?;

        i.handle(InitGraphQLUsers::default()).await?;
        i.handle(InitGraphQLProfiles::default()).await?;
        i.handle(InitGraphQLShows::default()).await?;
        i.handle(InitGraphQLEpisodes::default()).await?;

        Ok(())
    }
}
