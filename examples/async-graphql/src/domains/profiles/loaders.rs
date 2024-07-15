use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use derive_new::new;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;

use super::{model::Profile, service::Service};

/// A dataloader for `Profile` instances
#[derive(new)]
pub struct Loader {
    /// The SeaOrm database connection
    profiles: Arc<Box<dyn Service>>,
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
