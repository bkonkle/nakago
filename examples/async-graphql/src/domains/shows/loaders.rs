use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use derive_new::new;
use nakago::{provider, Inject, Provider, Tag};
use nakago_derive::Provider;

use super::{
    model::Show,
    service::{Service, SERVICE},
};

/// Tag(shows::Loader)
pub const LOADER: Tag<DataLoader<Loader>> = Tag::new("shows::Loader");

/// A dataloader for `Show` instances
#[derive(new)]
pub struct Loader {
    /// The SeaOrm database connection
    shows: Arc<Box<dyn Service>>,
}

#[async_trait]
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
///
/// **Provides:** `Arc<DataLoader<shows::Loader>>`
///
/// **Depends on:**
///  - `Tag(shows::Service)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<DataLoader<Loader>> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<DataLoader<Loader>>> {
        let service = i.get(&SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            Loader::new(service.clone()),
            tokio::spawn,
        )))
    }
}
