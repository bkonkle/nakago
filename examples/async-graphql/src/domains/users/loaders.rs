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
    model::User,
    service::{Service, SERVICE},
};

/// Tag(users::Loader)
pub const LOADER: Tag<DataLoader<Loader>> = Tag::new("users::Loader");

/// A dataloader for `User` instances
#[derive(new)]
pub struct Loader {
    /// The SeaOrm database connection
    locations: Arc<Box<dyn Service>>,
}

#[async_trait]
impl dataloader::Loader<String> for Loader {
    type Value = User;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let locations = self.locations.get_by_ids(keys.into()).await?;

        Ok(locations
            .into_iter()
            .map(|location| (location.id.clone(), location))
            .collect())
    }
}

/// Provide the Loader
///
/// **Provides:** `users::Loader`
///
/// **Depends on:**
///  - `Tag(users::Service)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<DataLoader<Loader>> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<DataLoader<Loader>>> {
        let users_service = i.get(&SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            Loader::new(users_service.clone()),
            tokio::spawn,
        )))
    }
}
