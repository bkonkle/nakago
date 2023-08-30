use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use super::{
    loaders::{ProvideRoleGrantLoader, ROLE_GRANT_LOADER},
    service::{ProvideRoleGrantsService, ROLE_GRANTS_SERVICE},
};

/// Provide dependencies needed for the RoleGrants domain
#[derive(Default)]
pub struct LoadRoleGrants {}

#[async_trait]
impl Hook for LoadRoleGrants {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&ROLE_GRANTS_SERVICE, ProvideRoleGrantsService::default())
            .await?;

        i.provide(&ROLE_GRANT_LOADER, ProvideRoleGrantLoader::default())
            .await?;

        Ok(())
    }
}
