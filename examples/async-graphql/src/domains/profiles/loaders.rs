use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{inject, Inject, Provider, Tag};
use nakago_derive::Provider;

use super::{
    model::Profile,
    service::{Service, SERVICE},
};

/// Tag(ProfileLoader)
pub const LOADER: Tag<DataLoader<Loader>> = Tag::new("ProfileLoader");

/// A dataloader for `Profile` instances
pub struct Loader {
    /// The SeaOrm database connection
    profiles: Arc<Box<dyn Service>>,
}

/// The default implementation for the `Loader`
impl Loader {
    /// Create a new instance
    pub fn new(profiles: Arc<Box<dyn Service>>) -> Self {
        Self { profiles }
    }
}

#[async_trait]
impl dataloader::Loader<String> for Loader {
    type Value = Profile;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let profiles = self.profiles.get_by_ids(keys.into()).await?;

        Ok(profiles
            .into_iter()
            .map(|profile| (profile.id.clone(), profile))
            .collect())
    }
}

/// Provide the Loader
///
/// **Provides:** `Loader`
///
/// **Depends on:**
///  - `Tag(Service)`
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
