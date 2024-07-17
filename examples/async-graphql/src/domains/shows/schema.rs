use async_graphql::dataloader::DataLoader;
use nakago::Inject;

use crate::domains::graphql::SchemaBuilder;

use super::{loaders, mutation, query, service, Loader, Mutation, Query, Service};

/// Provide dependencies needed for the Shows domain
pub async fn load(i: &Inject) -> nakago::Result<()> {
    i.provide::<Box<dyn Service>>(service::Provide::default())
        .await?;
    i.provide::<DataLoader<Loader>>(loaders::Provide::default())
        .await?;
    i.provide::<Query>(query::Provide::default()).await?;
    i.provide::<Mutation>(mutation::Provide::default()).await?;

    Ok(())
}

/// The Hook for initializing GraphQL User dependencies
pub async fn init(i: &Inject) -> nakago::Result<()> {
    let loader = i.get::<DataLoader<Loader>>().await?;

    i.modify::<SchemaBuilder, _>(|builder| Ok(builder.data(loader.clone())))
        .await?;

    Ok(())
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, EmptySubscription};
    use async_trait::async_trait;
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
            let service = i.get::<Box<dyn Service>>().await?;
            let role_grants = i.get::<Box<dyn role_grants::Service>>().await?;

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
