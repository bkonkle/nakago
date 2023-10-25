use async_trait::async_trait;
use nakago::{inject, Hook, Inject};

use crate::{domains::role_grants, graphql};

use super::{
    loaders::{self, LOADER},
    service::{self, SERVICE},
};

/// Provide dependencies needed for the Shows domain
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        i.provide(&SERVICE, service::Provide::default()).await?;

        i.provide(&LOADER, loaders::Provide::default()).await?;

        Ok(())
    }
}

/// The Hook for initializing the dependencies for the GraphQL Shows resolver
///
/// **Depends on:**
///  - Tag(RoleGrantsService)
///  - Tag(ShowsService)
///  - Tag(GraphQLSchemaBuilder)
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let role_grants = i.get(&role_grants::SERVICE).await?;
        let service = i.get(&SERVICE).await?;

        i.modify(&graphql::SCHEMA_BUILDER, |builder| {
            Ok(builder.data(role_grants.clone()).data(service.clone()))
        })
        .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, EmptySubscription};
    use nakago::{Provider, Tag};

    use crate::domains::shows::{Mutation, Query};

    use super::*;

    /// Tag(ShowsSchema)
    #[allow(dead_code)]
    pub const SCHEMA: Tag<Box<Schema>> = Tag::new("ShowsSchema");

    /// The Schema, covering just the Shows domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    ///
    /// **Provides:** `Arc<Schema>`
    ///
    /// **Depends on:**
    ///   - `Tag(ShowsService)`
    ///   - `Tag(ShowLoader)`
    #[derive(Default)]
    pub struct ProvideSchema {}

    #[async_trait]
    impl Provider<Schema> for ProvideSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Schema>> {
            let service = i.get(&SERVICE).await?;

            let schema: Schema =
                Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                    .data(service)
                    .finish();

            Ok(Arc::new(schema))
        }
    }
}
