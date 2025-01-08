use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use derive_new::new;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;

use super::{model::Show, service::Service};

/// A dataloader for `Show` instances
#[derive(new)]
pub struct Loader {
    /// The SeaOrm database connection
    shows: Arc<Box<dyn Service>>,
}

impl dataloader::Loader<String> for Loader {
    type Value = Show;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let shows = self.shows.get_by_ids(keys.into()).await?;

        Ok(shows
            .into_iter()
            .map(|show| (show.id.clone(), show))
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
        let service = i.get::<Box<dyn Service>>().await?;

        Ok(Arc::new(DataLoader::new(
            Loader::new(service.clone()),
            tokio::spawn,
        )))
    }
}
