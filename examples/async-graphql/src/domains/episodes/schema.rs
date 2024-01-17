use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};

use crate::domains::graphql;

use super::{loaders, mutation, query, service, LOADER, MUTATION, QUERY, SERVICE};

/// Provide dependencies needed for the Episodes domain
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide(&SERVICE, service::Provide::default()).await?;
        i.provide(&LOADER, loaders::Provide::default()).await?;
        i.provide(&QUERY, query::Provide::default()).await?;
        i.provide(&MUTATION, mutation::Provide::default()).await?;

        Ok(())
    }
}

/// The Hook for initializing GraphQL User dependencies
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let loader = i.get(&LOADER).await?;

        i.modify(&graphql::SCHEMA_BUILDER, |builder| Ok(builder.data(loader)))
            .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, dataloader::DataLoader, EmptySubscription};
    use nakago::{provider, Provider, Tag};
    use nakago_derive::Provider;

    use crate::domains::{
        episodes::{Mutation, Query},
        shows,
    };

    use super::*;

    /// Tag(episodes::Schema)
    #[allow(dead_code)]
    pub const SCHEMA: Tag<Schema> = Tag::new("episodes::Schema");

    /// The Schema, covering just the Episodes domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    ///
    /// **Provides:** `Arc<episodes::Schema>`
    ///
    /// **Depends on:**
    ///   - `Tag(episodes::Service)`
    ///   - `Tag(shows::Loader)`
    #[derive(Default)]
    pub struct Provide {}

    #[Provider]
    #[async_trait]
    impl Provider<Schema> for Provide {
        async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Schema>> {
            let service = i.get(&SERVICE).await?;
            let shows = i.get(&shows::SERVICE).await?;
            let show_loader = i.get(&shows::LOADER).await?;

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
