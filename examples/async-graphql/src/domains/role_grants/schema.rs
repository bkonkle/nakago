use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};

use crate::domains::graphql::SchemaBuilder;

use super::{loaders, service, Loader, Service};

/// Provide dependencies needed for the RoleGrants domain
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide::<Box<dyn Service>>(service::Provide::default())
            .await?;
        i.provide::<DataLoader<Loader>>(loaders::Provide::default())
            .await?;

        Ok(())
    }
}

/// The Hook for initializing GraphQL dependencies
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let loader = i.get::<DataLoader<Loader>>().await?;

        i.modify::<SchemaBuilder, _>(|builder| Ok(builder.data(loader.clone())))
            .await?;

        Ok(())
    }
}
