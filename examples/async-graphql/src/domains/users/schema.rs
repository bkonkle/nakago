use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};

use crate::domains::graphql::SchemaBuilder;

use super::{loaders, mutation, query, service, Loader, Mutation, Query, Service};

/// Provide dependencies needed for the Users domain
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

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, EmptySubscription};
    use nakago::{provider, Provider};

    use crate::domains::{
        profiles,
        users::{Mutation, Query},
    };

    use super::*;

    /// The Schema, covering just the Users domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    #[derive(Default)]
    pub struct Provide {}

    #[async_trait]
    impl Provider<Schema> for Provide {
        async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Schema>> {
            let service = i.get::<Box<dyn Service>>().await?;
            let profiles = i.get::<Box<dyn profiles::Service>>().await?;

            let schema: Schema = Schema::build(
                Query::default(),
                Mutation::new(service, profiles),
                EmptySubscription,
            )
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
