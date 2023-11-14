use std::sync::Arc;

use async_graphql::{self, EmptySubscription, MergedObject};
use async_trait::async_trait;
use nakago::{inject, Hook, Inject, Provider, Tag};
use nakago_async_graphql::schema;
use nakago_derive::Provider;

use crate::{authz::OSO, config::CONFIG};

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

/// Tag(graphql::Schema)
pub const SCHEMA: Tag<Schema> = Tag::new("graphql::Schema");

/// Tag(graphql::SchemaBuilder)
pub const SCHEMA_BUILDER: Tag<SchemaBuilder> = Tag::new("graphql::SchemaBuilder");

/// Provide the SchemaBuilder
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<SchemaBuilder> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<SchemaBuilder>> {
        let users_service = i.get(&users::SERVICE).await?;
        let profiles_service = i.get(&profiles::SERVICE).await?;
        let role_grants_service = i.get(&role_grants::SERVICE).await?;
        let shows_service = i.get(&shows::SERVICE).await?;
        let episodes_service = i.get(&episodes::SERVICE).await?;

        let users_query = users::Query::default();
        let profiles_query = profiles::Query::new(profiles_service.clone());
        let shows_query = shows::Query::new(shows_service.clone());
        let episodes_query = episodes::Query::new(episodes_service.clone());

        let users_mutation = users::Mutation::new(users_service, profiles_service.clone());
        let profiles_mutation = profiles::Mutation::new(profiles_service);
        let shows_mutation = shows::Mutation::new(shows_service.clone(), role_grants_service);
        let episodes_mutation = episodes::Mutation::new(episodes_service, shows_service);

        Ok(Arc::new(Schema::build(
            Query(users_query, profiles_query, shows_query, episodes_query),
            Mutation(
                users_mutation,
                profiles_mutation,
                shows_mutation,
                episodes_mutation,
            ),
            EmptySubscription,
        )))
    }
}

/// Loads the GraphQL schema builder dependencies
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

/// Initializes the GraphQL schema builder
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let config = i.get(&CONFIG).await?;
        let oso = i.get(&OSO).await?;

        i.modify(&SCHEMA_BUILDER, |builder| {
            Ok(builder.data(config.clone()).data((*oso).clone()))
        })
        .await?;

        i.handle(users::schema::Init::default()).await?;
        i.handle(profiles::schema::Init::default()).await?;
        i.handle(role_grants::schema::Init::default()).await?;
        i.handle(shows::schema::Init::default()).await?;
        i.handle(episodes::schema::Init::default()).await?;

        i.handle(
            schema::Init::default()
                .with_builder_tag(&SCHEMA_BUILDER)
                .with_schema_tag(&SCHEMA),
        )
        .await?;

        Ok(())
    }
}
