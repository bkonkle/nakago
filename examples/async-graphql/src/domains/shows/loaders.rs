use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};
use std::{collections::HashMap, sync::Arc};

use super::{
    model::Show,
    service::{ShowsService, SHOWS_SERVICE},
};

/// Tag(ShowLoader)
pub const SHOW_LOADER: Tag<DataLoader<ShowLoader>> = Tag::new("ShowLoader");

/// Provide the ShowLoader
///
/// **Provides:** `ShowLoader`
///
/// **Depends on:**
///  - `Tag(ShowsService)`
#[derive(Default)]
pub struct ProvideShowLoader {}

#[async_trait]
impl Provider for ProvideShowLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let shows_service = i.get(&SHOWS_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            ShowLoader::new(shows_service.clone()),
            tokio::spawn,
        )))
    }
}

/// A dataloader for `Show` instances
pub struct ShowLoader {
    /// The SeaOrm database connection
    shows: Arc<Box<dyn ShowsService>>,
}

/// The default implementation for the `ShowLoader`
impl ShowLoader {
    /// Create a new instance
    pub fn new(shows: Arc<Box<dyn ShowsService>>) -> Self {
        Self { shows }
    }
}

#[async_trait]
impl Loader<String> for ShowLoader {
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
