use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::inject;

use super::service::{DefaultEpisodesService, EpisodeLoader, EpisodesService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(EpisodesService)
pub const EPISODES_SERVICE: inject::Tag<Arc<dyn EpisodesService>> =
    inject::Tag::new("EpisodesService");

/// Provide the EpisodesService
///
/// **Provides:** `Arc<dyn EpisodesService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideEpisodesService {}

#[async_trait]
impl inject::Provider<Arc<dyn EpisodesService>> for ProvideEpisodesService {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<dyn EpisodesService>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(DefaultEpisodesService::new(db.clone())))
    }
}

/// Tag(EpisodeLoader)
pub const EPISODE_LOADER: inject::Tag<Arc<DataLoader<EpisodeLoader>>> =
    inject::Tag::new("EpisodeLoader");

/// Provide the EpisodeLoader
///
/// **Provides:** `EpisodeLoader`
///
/// **Depends on:**
///  - `Tag(EpisodesService)`
#[derive(Default)]
pub struct ProvideEpisodeLoader {}

#[async_trait]
impl inject::Provider<Arc<DataLoader<EpisodeLoader>>> for ProvideEpisodeLoader {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<DataLoader<EpisodeLoader>>> {
        let episodes_service = i.get(&EPISODES_SERVICE)?;

        Ok(Arc::new(DataLoader::new(
            EpisodeLoader::new(episodes_service.clone()),
            tokio::spawn,
        )))
    }
}
