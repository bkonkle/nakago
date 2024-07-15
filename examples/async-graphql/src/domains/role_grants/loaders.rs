use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{self, DataLoader},
    FieldError,
};
use async_trait::async_trait;
use derive_new::new;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;

use super::{model::RoleGrant, service::Service};

/// A dataloader for `RoleGrant` instances
#[derive(new)]
pub struct Loader {
    /// The SeaOrm database connection
    role_grants: Arc<Box<dyn Service>>,
}

#[async_trait]
impl dataloader::Loader<String> for Loader {
    type Value = RoleGrant;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let role_grants = self.role_grants.get_by_ids(keys.into()).await?;

        Ok(role_grants
            .into_iter()
            .map(|role_grant| (role_grant.id.clone(), role_grant))
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
