use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};

use crate::domains::graphql::SchemaBuilder;

use super::{loaders, mutation, query, service, Loader, Mutation, Query, Service};

/// Provide dependencies needed for the Shows domain
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide_type::<Box<dyn Service>>(service::Provide::default())
            .await?;
        i.provide_type::<DataLoader<Loader>>(loaders::Provide::default())
            .await?;
        i.provide_type::<Query>(query::Provide::default()).await?;
        i.provide_type::<Mutation>(mutation::Provide::default())
            .await?;

        Ok(())
    }
}

/// The Hook for initializing GraphQL User dependencies
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let loader = i.get_type::<Loader>().await?;

        i.modify_type::<SchemaBuilder, _>(|builder| Ok(builder.data(loader.clone())))
            .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, EmptySubscription};
    use nakago::{provider, Provider};
    use service::Service;

    use crate::domains::{
        role_grants,
        shows::{Mutation, Query},
    };

    use super::*;

    /// The Schema, covering just the Shows domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    #[derive(Default)]
    pub struct ProvideSchema {}

    #[async_trait]
    impl Provider<Schema> for ProvideSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Schema>> {
            let service = i.get_type::<Box<dyn Service>>().await?;
            let role_grants = i.get_type::<Box<dyn role_grants::Service>>().await?;

            let schema: Schema = Schema::build(
                Query::new(service.clone()),
                Mutation::new(service, role_grants),
                EmptySubscription,
            )
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
