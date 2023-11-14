use async_trait::async_trait;
use nakago::{inject, Hook, Inject};

use crate::domains::graphql;

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

/// The Hook for initializing GraphQL dependencies
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let loader = i.get(&LOADER).await?;

        i.modify(&graphql::SCHEMA_BUILDER, |builder| {
            Ok(builder.data(loader.clone()))
        })
        .await?;

        Ok(())
    }
}
