use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};

use crate::domains::graphql;

use super::{loaders, mutation, query, service, LOADER, MUTATION, QUERY, SERVICE};

/// Provide dependencies needed for the Shows domain
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

    use async_graphql::{self, EmptySubscription};
    use nakago::{provider, Provider, Tag};

    use crate::domains::{
        role_grants,
        shows::{Mutation, Query},
    };

    use super::*;

    /// Tag(shows::Schema)
    #[allow(dead_code)]
    pub const SCHEMA: Tag<Box<Schema>> = Tag::new("shows::Schema");

    /// The Schema, covering just the Shows domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    ///
    /// **Provides:** `Arc<shows::Schema>`
    ///
    /// **Depends on:**
    ///   - `Tag(shows::Service)`
    #[derive(Default)]
    pub struct ProvideSchema {}

    #[async_trait]
    impl Provider<Schema> for ProvideSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Schema>> {
            let service = i.get(&SERVICE).await?;
            let role_grants = i.get(&role_grants::SERVICE).await?;

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
