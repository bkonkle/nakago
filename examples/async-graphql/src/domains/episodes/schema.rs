use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use crate::{
    domains::shows::{loaders::SHOW_LOADER, service::SHOWS_SERVICE},
    graphql::GRAPHQL_SCHEMA_BUILDER,
};

use super::{
    loaders::{ProvideEpisodeLoader, EPISODE_LOADER},
    service::{ProvideEpisodesService, EPISODES_SERVICE},
};

/// Provide dependencies needed for the Episodes domain
#[derive(Default)]
pub struct LoadEpisodes {}

#[async_trait]
impl Hook for LoadEpisodes {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&EPISODES_SERVICE, ProvideEpisodesService::default())
            .await?;

        i.provide(&EPISODE_LOADER, ProvideEpisodeLoader::default())
            .await?;

        Ok(())
    }
}

/// The Hook for initializing the dependencies for the GraphQL Episodes resolver
///
/// **Depends on:**
///  - Tag(EpisodesService)
///  - Tag(ShowsService)
///  - Tag(ShowLoader)
///  - Tag(GraphQLSchemaBuilder)
#[derive(Default)]
pub struct InitGraphQLEpisodes {}

#[async_trait]
impl Hook for InitGraphQLEpisodes {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let episodes = i.get(&EPISODES_SERVICE).await?;
        let shows = i.get(&SHOWS_SERVICE).await?;
        let show_loader = i.get(&SHOW_LOADER).await?;

        let builder = i.consume(&GRAPHQL_SCHEMA_BUILDER).await?;

        i.inject(
            &GRAPHQL_SCHEMA_BUILDER,
            builder
                .data(shows.clone())
                .data(show_loader.clone())
                .data(episodes.clone()),
        )
        .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{dataloader::DataLoader, EmptySubscription, Schema};
    use nakago::{Provider, Tag};
    use nakago_derive::Provider;

    use crate::domains::episodes::resolver::{EpisodesMutation, EpisodesQuery};

    use super::*;

    /// Tag(EpisodesSchema)
    #[allow(dead_code)]
    pub const EPISODES_SCHEMA: Tag<EpisodesSchema> = Tag::new("EpisodesSchema");

    /// The EpisodesSchema, covering just the Episodes domain. Useful for testing in isolation.
    pub type EpisodesSchema = Schema<EpisodesQuery, EpisodesMutation, EmptySubscription>;

    /// Provide the EpisodesSchema
    ///
    /// **Provides:** `Arc<EpisodesSchema>`
    ///
    /// **Depends on:**
    ///   - `Tag(EpisodesService)`
    ///   - `Tag(ShowLoader)`
    #[derive(Default)]
    pub struct ProvideEpisodesSchema {}

    #[Provider]
    #[async_trait]
    impl Provider<EpisodesSchema> for ProvideEpisodesSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<EpisodesSchema>> {
            let service = i.get(&EPISODES_SERVICE).await?;
            let show_loader = i.get(&SHOW_LOADER).await?;

            let schema: EpisodesSchema = Schema::build(
                EpisodesQuery::default(),
                EpisodesMutation::default(),
                EmptySubscription,
            )
            .data(service)
            .data(DataLoader::new(show_loader, tokio::spawn))
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
