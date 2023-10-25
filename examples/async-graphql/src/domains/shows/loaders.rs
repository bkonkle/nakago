use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{inject, Inject, Provider, Tag};
use nakago_derive::Provider;

use super::{
    model::Show,
    service::{Service, SERVICE},
};

/// Tag(ShowLoader)
pub const LOADER: Tag<DataLoader<Loader>> = Tag::new("ShowLoader");

/// A dataloader for `Show` instances
pub struct Loader {
    /// The SeaOrm database connection
    shows: Arc<Box<dyn Service>>,
}

/// The default implementation for the `Loader`
impl Loader {
    /// Create a new instance
    pub fn new(shows: Arc<Box<dyn Service>>) -> Self {
        Self { shows }
    }
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
/// **Provides:** `Loader`
///
/// **Depends on:**
///  - `Tag(ShowsService)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<DataLoader<Loader>> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<DataLoader<Loader>>> {
        let service = i.get(&SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            Loader::new(service.clone()),
            tokio::spawn,
        )))
    }
}
