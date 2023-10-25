use async_trait::async_trait;
use nakago::{inject, Hook, Inject};

use super::{
    loaders::{self, LOADER},
    service::{self, SERVICE},
};

/// Provide dependencies needed for the RoleGrants domain
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        i.provide(&SERVICE, service::Provide::default()).await?;

        i.provide(&LOADER, loaders::Provide::default()).await?;

        Ok(())
    }
}
