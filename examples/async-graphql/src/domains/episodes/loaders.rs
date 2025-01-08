use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use derive_new::new;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;

use super::{model::Episode, service::Service};

/// A dataloader for `Episode` instances
#[derive(new)]
pub struct Loader {
    /// The SeaOrm database connection
    episodes: Arc<Box<dyn Service>>,
}

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
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<DataLoader<Loader>> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<DataLoader<Loader>>> {
        let episodes_service = i.get::<Box<dyn Service>>().await?;

        Ok(Arc::new(DataLoader::new(
            Loader::new(episodes_service.clone()),
            tokio::spawn,
        )))
    }
}
