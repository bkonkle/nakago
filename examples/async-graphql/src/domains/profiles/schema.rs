use async_graphql::dataloader::DataLoader;
use nakago::Inject;

use crate::domains::graphql::SchemaBuilder;

use super::{loaders, mutation, query, service, Loader, Mutation, Query, Service};

/// Provide dependencies needed for the Profiles domain
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

    use async_graphql::{self, dataloader::DataLoader, EmptySubscription};
    use async_trait::async_trait;
    use nakago::{provider, Provider};

    use crate::domains::{
        profiles::{Mutation, Query},
        users,
    };

    use super::*;

    /// The Schema, covering just the Profiles domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    #[derive(Default)]
    pub struct Provide {}

    #[async_trait]
    impl Provider<Schema> for Provide {
        async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Schema>> {
            let service = i.get::<Box<dyn Service>>().await?;
            let user_loader = i.get::<DataLoader<users::Loader>>().await?;

            let schema: Schema = Schema::build(
                Query::new(service.clone()),
                Mutation::new(service),
                EmptySubscription,
            )
            .data(DataLoader::new(user_loader, tokio::spawn))
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
