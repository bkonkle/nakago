use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provide, Tag};

use super::{
    model::RoleGrant,
    service::{Service, ROLE_GRANTS_SERVICE},
};

/// Tag(RoleGrantLoader)
pub const ROLE_GRANT_LOADER: Tag<DataLoader<RoleGrantLoader>> = Tag::new("RoleGrantLoader");

/// Provide the RoleGrantLoader
///
/// **Provides:** `RoleGrantLoader`
///
/// **Depends on:**
///  - `Tag(RoleGrantsService)`
#[derive(Default)]
pub struct Provider {}

#[async_trait]
impl Provide<DataLoader<RoleGrantLoader>> for Provider {
    async fn provide(&self, i: &Inject) -> InjectResult<DataLoader<RoleGrantLoader>> {
        let role_grants_service = i.get(&ROLE_GRANTS_SERVICE)?;

        Ok(DataLoader::new(
            RoleGrantLoader::new(role_grants_service.clone()),
            tokio::spawn,
        ))
    }
}

/// A dataloader for `RoleGrant` instances
pub struct RoleGrantLoader {
    /// The SeaOrm database connection
    role_grants: Arc<dyn Service>,
}

/// The default implementation for the `RoleGrantLoader`
impl RoleGrantLoader {
    /// Create a new instance
    pub fn new(role_grants: Arc<dyn Service>) -> Self {
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
