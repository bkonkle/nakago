use async_graphql::{dataloader::DataLoader, EmptySubscription, Schema};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provider, Tag};
use std::sync::Arc;

use crate::domains::{
    episodes::{
        resolver::{EpisodesMutation, EpisodesQuery},
        service::EPISODES_SERVICE,
    },
    shows::loaders::SHOW_LOADER,
};

/// Tag(EpisodesSchema)
#[allow(dead_code)]
pub const EPISODES_SCHEMA: Tag<Box<EpisodesSchema>> = Tag::new("EpisodesSchema");

/// The EpisodesSchema, covering just the Episodes domain
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
