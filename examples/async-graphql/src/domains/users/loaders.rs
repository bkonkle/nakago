use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{inject, Inject, Provider, Tag};
use nakago_derive::Provider;

use super::{
    model::User,
    service::{Service, SERVICE},
};

/// Tag(UserLoader)
pub const LOADER: Tag<DataLoader<Loader>> = Tag::new("UserLoader");

/// A dataloader for `User` instances
pub struct Loader {
    /// The SeaOrm database connection
    locations: Arc<Box<dyn Service>>,
}

/// The default implementation for the `Loader`
impl Loader {
    /// Create a new instance
    pub fn new(locations: Arc<Box<dyn Service>>) -> Self {
        Self { locations }
    }
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
/// **Provides:** `Loader`
///
/// **Depends on:**
///  - `Tag(UsersService)`
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
