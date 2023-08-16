use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provider, Tag};

use super::{
    model::Episode,
    service::{EpisodesService, EPISODES_SERVICE},
};

/// Tag(EpisodeLoader)
pub const EPISODE_LOADER: Tag<DataLoader<EpisodeLoader>> = Tag::new("EpisodeLoader");

/// A dataloader for `Episode` instances
pub struct EpisodeLoader {
    /// The SeaOrm database connection
    episodes: Arc<Box<dyn EpisodesService>>,
}

/// The default implementation for the `EpisodeLoader`
impl EpisodeLoader {
    /// Create a new instance
    pub fn new(episodes: Arc<Box<dyn EpisodesService>>) -> Self {
        Self { episodes }
    }
}

#[async_trait]
impl Loader<String> for EpisodeLoader {
    type Value = Episode;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let episodes = self.episodes.get_by_ids(keys.into()).await?;

        Ok(episodes
            .into_iter()
            .map(|episode| (episode.id.clone(), episode))
            .collect())
    }
}

/// Provide the EpisodeLoader
///
/// **Provides:** `EpisodeLoader`
///
/// **Depends on:**
///  - `Tag(EpisodesService)`
#[derive(Default)]
pub struct ProvideEpisodeLoader {}

#[async_trait]
impl Provider<DataLoader<EpisodeLoader>> for ProvideEpisodeLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<DataLoader<EpisodeLoader>>> {
        let episodes_service = i.get(&EPISODES_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            EpisodeLoader::new(episodes_service.clone()),
            tokio::spawn,
        )))
    }
}
