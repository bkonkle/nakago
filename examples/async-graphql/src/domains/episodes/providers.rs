use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};

use super::service::{DefaultEpisodesService, EpisodeLoader, EpisodesService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(EpisodesService)
pub const EPISODES_SERVICE: Tag<Box<dyn EpisodesService>> = Tag::new("EpisodesService");

/// Provide the EpisodesService
///
/// **Provides:** `Arc<dyn EpisodesService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideEpisodesService {}

#[async_trait]
impl Provider for ProvideEpisodesService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        let service: Box<dyn EpisodesService> = Box::new(DefaultEpisodesService::new(db.clone()));

        Ok(Arc::new(service))
    }
}

/// Tag(EpisodeLoader)
pub const EPISODE_LOADER: Tag<DataLoader<EpisodeLoader>> = Tag::new("EpisodeLoader");

/// Provide the EpisodeLoader
///
/// **Provides:** `EpisodeLoader`
///
/// **Depends on:**
///  - `Tag(EpisodesService)`
#[derive(Default)]
pub struct ProvideEpisodeLoader {}

#[async_trait]
impl Provider for ProvideEpisodeLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let episodes_service = i.get(&EPISODES_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            EpisodeLoader::new(episodes_service.clone()),
            tokio::spawn,
        )))
    }
}
