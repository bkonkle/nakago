use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provide, Tag};

use super::{
    model::Episode,
    service::{Service, EPISODES_SERVICE},
};

/// Tag(EpisodeLoader)
pub const EPISODE_LOADER: Tag<DataLoader<EpisodeLoader>> = Tag::new("EpisodeLoader");

/// Provide the EpisodeLoader
///
/// **Provides:** `EpisodeLoader`
///
/// **Depends on:**
///  - `Tag(EpisodesService)`
#[derive(Default)]
pub struct Provider {}

#[async_trait]
impl Provide<DataLoader<EpisodeLoader>> for Provider {
    async fn provide(&self, i: &Inject) -> InjectResult<DataLoader<EpisodeLoader>> {
        let episodes_service = i.get(&EPISODES_SERVICE)?;

        Ok(DataLoader::new(
            EpisodeLoader {
                episodes: episodes_service.clone(),
            },
            tokio::spawn,
        ))
    }
}

/// A dataloader for `Episode` instances
pub struct EpisodeLoader {
    /// The SeaOrm database connection
    episodes: Arc<dyn Service>,
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
