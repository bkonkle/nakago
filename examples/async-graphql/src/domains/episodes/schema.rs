use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};

use crate::domains::graphql::SchemaBuilder;

use super::{loaders, mutation, query, service, Loader, Mutation, Query, Service};

/// Provide dependencies needed for the Episodes domain
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide::<Box<dyn Service>>(service::Provide::default())
            .await?;
        i.provide::<DataLoader<Loader>>(loaders::Provide::default())
            .await?;
        i.provide::<Query>(query::Provide::default()).await?;
        i.provide::<Mutation>(mutation::Provide::default()).await?;

        Ok(())
    }
}

/// The Hook for initializing GraphQL User dependencies
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let loader = i.get::<DataLoader<Loader>>().await?;

        i.modify::<SchemaBuilder, _>(|builder| Ok(builder.data(loader)))
            .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, dataloader::DataLoader, EmptySubscription};
    use nakago::{provider, Provider};
    use nakago_derive::Provider;

    use crate::domains::{
        episodes::{Mutation, Query},
        shows,
    };

    use super::*;

    /// The Schema, covering just the Episodes domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    #[derive(Default)]
    pub struct Provide {}

    #[Provider]
    #[async_trait]
    impl Provider<Schema> for Provide {
        async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Schema>> {
            let service = i.get::<Box<dyn Service>>().await?;
            let shows = i.get::<Box<dyn shows::Service>>().await?;
            let show_loader = i.get::<DataLoader<shows::Loader>>().await?;

            let schema: Schema = Schema::build(
                Query::new(service.clone()),
                Mutation::new(service, shows),
                EmptySubscription,
            )
            .data(DataLoader::new(show_loader, tokio::spawn))
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
