use async_graphql::dataloader::DataLoader;
use nakago::Inject;

use crate::domains::graphql::SchemaBuilder;

use super::{loaders, service, Loader, Service};

/// Provide dependencies needed for the RoleGrants domain
pub async fn load(i: &Inject) -> nakago::Result<()> {
    i.provide::<Box<dyn Service>>(service::Provide::default())
        .await?;
    i.provide::<DataLoader<Loader>>(loaders::Provide::default())
        .await?;

    Ok(())
}

/// The Hook for initializing GraphQL dependencies
pub async fn init(i: &Inject) -> nakago::Result<()> {
    let loader = i.get::<DataLoader<Loader>>().await?;

    i.modify::<SchemaBuilder, _>(|builder| Ok(builder.data(loader.clone())))
        .await?;

    Ok(())
}
