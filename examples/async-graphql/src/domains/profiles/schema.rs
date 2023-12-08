use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};

use crate::domains::graphql;

use super::{
    loaders::{self, LOADER},
    mutation, query,
    service::{self, SERVICE},
    MUTATION, QUERY,
};

/// Provide dependencies needed for the Profiles domain
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

        i.modify(&graphql::SCHEMA_BUILDER, |builder| {
            Ok(builder.data(loader.clone()))
        })
        .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, dataloader::DataLoader, EmptySubscription};
    use nakago::{provider, Provider, Tag};

    use crate::domains::{
        profiles::{Mutation, Query},
        users,
    };

    use super::*;

    /// Tag(profiles::Schema)
    #[allow(dead_code)]
    pub const SCHEMA: Tag<Box<Schema>> = Tag::new("profiles::Schema");

    /// The Schema, covering just the Profiles domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    ///
    /// **Provides:** `Arc<profiles::Schema>`
    ///
    /// **Depends on:**
    ///   - `Tag(profiles::Service)`
    ///   - `Tag(users::Loader)`
    #[derive(Default)]
    pub struct Provide {}

    #[async_trait]
    impl Provider<Schema> for Provide {
        async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Schema>> {
            let service = i.get(&SERVICE).await?;
            let user_loader = i.get(&users::LOADER).await?;

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
