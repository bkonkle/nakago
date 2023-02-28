use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::inject;

use super::service::{DefaultRoleGrantsService, RoleGrantLoader, RoleGrantsService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(RoleGrantsService)
pub const ROLE_GRANTS_SERVICE: inject::Tag<Arc<dyn RoleGrantsService>> =
    inject::Tag::new("RoleGrantsService");

/// Provide the RoleGrantsService
///
/// **Provides:** `Arc<dyn RoleGrantsService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideRoleGrantsService {}

#[async_trait]
impl inject::Provider<Arc<dyn RoleGrantsService>> for ProvideRoleGrantsService {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<dyn RoleGrantsService>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(DefaultRoleGrantsService::new(db.clone())))
    }
}

/// Tag(RoleGrantLoader)
pub const ROLE_GRANT_LOADER: inject::Tag<DataLoader<RoleGrantLoader>> =
    inject::Tag::new("RoleGrantLoader");

/// Provide the RoleGrantLoader
///
/// **Provides:** `RoleGrantLoader`
///
/// **Depends on:**
///  - `Tag(RoleGrantsService)`
#[derive(Default)]
pub struct ProvideRoleGrantLoader {}

#[async_trait]
impl inject::Provider<DataLoader<RoleGrantLoader>> for ProvideRoleGrantLoader {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<DataLoader<RoleGrantLoader>> {
        let role_grants_service = i.get(&ROLE_GRANTS_SERVICE)?;

        Ok(DataLoader::new(
            RoleGrantLoader::new(role_grants_service.clone()),
            tokio::spawn,
        ))
    }
}
