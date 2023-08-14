use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};

use super::service::{DefaultRoleGrantsService, RoleGrantLoader, RoleGrantsService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(RoleGrantsService)
pub const ROLE_GRANTS_SERVICE: Tag<Box<dyn RoleGrantsService>> = Tag::new("RoleGrantsService");

/// Provide the RoleGrantsService
///
/// **Provides:** `Arc<dyn RoleGrantsService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideRoleGrantsService {}

#[async_trait]
impl Provider for ProvideRoleGrantsService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        let service: Box<dyn RoleGrantsService> =
            Box::new(DefaultRoleGrantsService::new(db.clone()));

        Ok(Arc::new(service))
    }
}

/// Tag(RoleGrantLoader)
pub const ROLE_GRANT_LOADER: Tag<DataLoader<RoleGrantLoader>> = Tag::new("RoleGrantLoader");

/// Provide the RoleGrantLoader
///
/// **Provides:** `RoleGrantLoader`
///
/// **Depends on:**
///  - `Tag(RoleGrantsService)`
#[derive(Default)]
pub struct ProvideRoleGrantLoader {}

#[async_trait]
impl Provider for ProvideRoleGrantLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let role_grants_service = i.get(&ROLE_GRANTS_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            RoleGrantLoader::new(role_grants_service.clone()),
            tokio::spawn,
        )))
    }
}
