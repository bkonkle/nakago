use async_trait::async_trait;
use nakago::{inject, Hook, Inject};

use crate::{domains::shows, graphql};

use super::{
    loaders::{self, LOADER},
    service::{self, SERVICE},
};

/// Provide dependencies needed for the Episodes domain
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

/// The Hook for initializing the dependencies for the GraphQL Episodes resolver
///
/// **Depends on:**
///  - Tag(EpisodesService)
///  - Tag(ShowsService)
///  - Tag(ShowLoader)
///  - Tag(GraphQLSchemaBuilder)
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let episodes = i.get(&SERVICE).await?;
        let shows = i.get(&shows::SERVICE).await?;
        let show_loader = i.get(&shows::LOADER).await?;

        i.modify(&graphql::SCHEMA_BUILDER, |builder| {
            Ok(builder
                .data(shows.clone())
                .data(show_loader.clone())
                .data(episodes.clone()))
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
    use nakago_derive::Provider;

    use crate::domains::episodes::resolver::{Mutation, Query};

    use super::*;

    /// Tag(Schema)
    #[allow(dead_code)]
    pub const SCHEMA: Tag<Schema> = Tag::new("EpisodesSchema");

    /// The Schema, covering just the Episodes domain. Useful for testing in isolation.
    pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    /// Provide the Schema
    ///
    /// **Provides:** `Arc<Schema>`
    ///
    /// **Depends on:**
    ///   - `Tag(EpisodesService)`
    ///   - `Tag(ShowLoader)`
    #[derive(Default)]
    pub struct Provide {}

    #[Provider]
    #[async_trait]
    impl Provider<Schema> for Provide {
        async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Schema>> {
            let service = i.get(&SERVICE).await?;
            let show_loader = i.get(&shows::LOADER).await?;

            let schema: Schema =
                Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                    .data(service)
                    .data(DataLoader::new(show_loader, tokio::spawn))
                    .finish();

            Ok(Arc::new(schema))
        }
    }
}
