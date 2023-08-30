use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use crate::{domains::role_grants::service::ROLE_GRANTS_SERVICE, graphql::GRAPHQL_SCHEMA_BUILDER};

use super::{
    loaders::{ProvideShowLoader, SHOW_LOADER},
    service::{ProvideShowsService, SHOWS_SERVICE},
};

/// Provide dependencies needed for the Shows domain
#[derive(Default)]
pub struct LoadShows {}

#[async_trait]
impl Hook for LoadShows {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&SHOWS_SERVICE, ProvideShowsService::default())
            .await?;

        i.provide(&SHOW_LOADER, ProvideShowLoader::default())
            .await?;

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
pub struct InitGraphQLShows {}

#[async_trait]
impl Hook for InitGraphQLShows {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let role_grants = i.get(&ROLE_GRANTS_SERVICE).await?;
        let shows = i.get(&SHOWS_SERVICE).await?;

        let builder = i.consume(&GRAPHQL_SCHEMA_BUILDER).await?;

        i.inject(
            &GRAPHQL_SCHEMA_BUILDER,
            builder.data(role_grants.clone()).data(shows.clone()),
        )
        .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{EmptySubscription, Schema};
    use nakago::{Provider, Tag};

    use crate::domains::shows::resolver::{ShowsMutation, ShowsQuery};

    use super::*;

    /// Tag(ShowsSchema)
    #[allow(dead_code)]
    pub const SHOWS_SCHEMA: Tag<Box<ShowsSchema>> = Tag::new("ShowsSchema");

    /// The ShowsSchema, covering just the Shows domain. Useful for testing in isolation.
    pub type ShowsSchema = Schema<ShowsQuery, ShowsMutation, EmptySubscription>;

    /// Provide the ShowsSchema
    ///
    /// **Provides:** `Arc<ShowsSchema>`
    ///
    /// **Depends on:**
    ///   - `Tag(ShowsService)`
    ///   - `Tag(ShowLoader)`
    #[derive(Default)]
    pub struct ProvideShowsSchema {}

    #[async_trait]
    impl Provider<ShowsSchema> for ProvideShowsSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<ShowsSchema>> {
            let service = i.get(&SHOWS_SERVICE).await?;

            let schema: ShowsSchema = Schema::build(
                ShowsQuery::default(),
                ShowsMutation::default(),
                EmptySubscription,
            )
            .data(service)
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
