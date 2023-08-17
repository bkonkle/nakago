use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;

use super::{
    model::RoleGrant,
    service::{RoleGrantsService, ROLE_GRANTS_SERVICE},
};

/// Tag(RoleGrantLoader)
pub const ROLE_GRANT_LOADER: Tag<DataLoader<RoleGrantLoader>> = Tag::new("RoleGrantLoader");

/// A dataloader for `RoleGrant` instances
pub struct RoleGrantLoader {
    /// The SeaOrm database connection
    role_grants: Arc<Box<dyn RoleGrantsService>>,
}

/// The default implementation for the `RoleGrantLoader`
impl RoleGrantLoader {
    /// Create a new instance
    pub fn new(role_grants: Arc<Box<dyn RoleGrantsService>>) -> Self {
        Self { role_grants }
    }
}

#[async_trait]
impl Loader<String> for RoleGrantLoader {
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

/// Provide the RoleGrantLoader
///
/// **Provides:** `RoleGrantLoader`
///
/// **Depends on:**
///  - `Tag(RoleGrantsService)`
#[derive(Default)]
pub struct ProvideRoleGrantLoader {}

#[Provider]
#[async_trait]
impl Provider<DataLoader<RoleGrantLoader>> for ProvideRoleGrantLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<DataLoader<RoleGrantLoader>>> {
        let role_grants_service = i.get(&ROLE_GRANTS_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            RoleGrantLoader::new(role_grants_service.clone()),
            tokio::spawn,
        )))
    }
}
