use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use derive_new::new;
use nakago::{inject, Inject, Provider, Tag};
use nakago_derive::Provider;

use super::{
    model::Episode,
    service::{Service, SERVICE},
};

/// Tag(episodes::Loader)
pub const LOADER: Tag<DataLoader<Loader>> = Tag::new("episodes::Loader");

/// A dataloader for `Episode` instances
#[derive(new)]
pub struct Loader {
    /// The SeaOrm database connection
    episodes: Arc<Box<dyn Service>>,
}

#[async_trait]
impl dataloader::Loader<String> for Loader {
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

/// Provide the Loader
///
/// **Provides:** `Arc<DataLoader<episodes::Loader>>`
///
/// **Depends on:**
///  - `Tag(episodes::Service)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<DataLoader<Loader>> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<DataLoader<Loader>>> {
        let episodes_service = i.get(&SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            Loader::new(episodes_service.clone()),
            tokio::spawn,
        )))
    }
}
