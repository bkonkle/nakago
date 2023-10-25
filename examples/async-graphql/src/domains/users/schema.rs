use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use crate::{domains::profiles, graphql};

use super::{
    loaders::{self, LOADER},
    service::{self, SERVICE},
};

/// Provide dependencies needed for the Users domain
#[derive(Default)]
pub struct LoadUsers {}

#[async_trait]
impl Hook for LoadUsers {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&SERVICE, service::Provide::default()).await?;

        i.provide(&LOADER, loaders::Provide::default()).await?;

        Ok(())
    }
}

/// The Hook for initializing the dependencies for the GraphQL Users resolver
///
/// **Depends on:**
///  - Tag(UsersService)
///  - Tag(ProfilesService)
///  - Tag(GraphQLSchemaBuilder)
#[derive(Default)]
pub struct InitGraphQLUsers {}

#[async_trait]
impl Hook for InitGraphQLUsers {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let service = i.get(&SERVICE).await?;
        let profiles = i.get(&profiles::SERVICE).await?;

        i.modify(&graphql::SCHEMA_BUILDER, |builder| {
            Ok(builder.data(service.clone()).data(profiles.clone()))
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

    use crate::domains::users::resolver::{Mutation, Query};

    use super::*;

    /// Tag(UsersSchema)
    #[allow(dead_code)]
    pub const SCHEMA: Tag<Box<Schema>> = Tag::new("UsersSchema");

    /// The Schema, covering just the Users domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    ///
    /// **Provides:** `Arc<Schema>`
    ///
    /// **Depends on:**
    ///   - `Tag(UsersService)`
    ///   - `Tag(ProfilesService)`
    #[derive(Default)]
    pub struct ProvideSchema {}

    #[async_trait]
    impl Provider<Schema> for ProvideSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Schema>> {
            let service = i.get(&SERVICE).await?;
            let profiles = i.get(&profiles::SERVICE).await?;

            let schema: Schema =
                Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                    .data(service)
                    .data(profiles)
                    .finish();

            Ok(Arc::new(schema))
        }
    }
}
