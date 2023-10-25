use async_trait::async_trait;
use nakago::{inject, Hook, Inject};

use crate::{domains::users, graphql};

use super::{
    loaders::{self, LOADER},
    service::{self, SERVICE},
};

/// Provide dependencies needed for the Profiles domain
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

/// The Hook for initializing the dependencies for the GraphQL Profiles resolver
///
/// **Depends on:**
///  - Tag(ProfilesService)
///  - Tag(UserLoader)
///  - Tag(GraphQLSchemaBuilder)
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let profiles = i.get(&SERVICE).await?;
        let user_loader = i.get(&users::LOADER).await?;

        i.modify(&graphql::SCHEMA_BUILDER, |builder| {
            Ok(builder.data(profiles.clone()).data(user_loader.clone()))
        })
        .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{self, dataloader::DataLoader, EmptySubscription};
    use nakago::{Provider, Tag};

    use crate::domains::profiles::{Mutation, Query};

    use super::*;

    /// Tag(Schema)
    #[allow(dead_code)]
    pub const SCHEMA: Tag<Box<Schema>> = Tag::new("ProfilesSchema");

    /// The Schema, covering just the Profiles domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    ///
    /// **Provides:** `Arc<Schema>`
    ///
    /// **Depends on:**
    ///   - `Tag(ProfilesService)`
    ///   - `Tag(ShowLoader)`
    #[derive(Default)]
    pub struct Provide {}

    #[async_trait]
    impl Provider<Schema> for Provide {
        async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Schema>> {
            let service = i.get(&SERVICE).await?;
            let user_loader = i.get(&users::LOADER).await?;

            let schema: Schema =
                Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                    .data(service)
                    .data(DataLoader::new(user_loader, tokio::spawn))
                    .finish();

            Ok(Arc::new(schema))
        }
    }
}
