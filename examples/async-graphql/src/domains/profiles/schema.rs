use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use crate::{
    domains::{profiles::service::PROFILES_SERVICE, users::loaders::USER_LOADER},
    graphql::GRAPHQL_SCHEMA_BUILDER,
};

/// The Hook for initializing the dependencies for the GraphQL Profiles resolver
///
/// **Depends on:**
///  - Tag(ProfilesService)
///  - Tag(UserLoader)
///  - Tag(GraphQLSchemaBuilder)
#[derive(Default)]
pub struct InitGraphQLProfiles {}

#[async_trait]
impl Hook for InitGraphQLProfiles {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let profiles = i.get(&PROFILES_SERVICE).await?;
        let user_loader = i.get(&USER_LOADER).await?;

        let builder = i.consume(&GRAPHQL_SCHEMA_BUILDER).await?;

        i.inject(
            &GRAPHQL_SCHEMA_BUILDER,
            builder.data(profiles.clone()).data(user_loader.clone()),
        )
        .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{dataloader::DataLoader, EmptySubscription, Schema};
    use nakago::{Provider, Tag};

    use crate::domains::profiles::resolver::{ProfilesMutation, ProfilesQuery};

    use super::*;

    /// Tag(ProfilesSchema)
    #[allow(dead_code)]
    pub const PROFILES_SCHEMA: Tag<Box<ProfilesSchema>> = Tag::new("ProfilesSchema");

    /// The ProfilesSchema, covering just the Profiles domain. Useful for testing in isolation.
    pub type ProfilesSchema = Schema<ProfilesQuery, ProfilesMutation, EmptySubscription>;

    /// Provide the ProfilesSchema
    ///
    /// **Provides:** `Arc<ProfilesSchema>`
    ///
    /// **Depends on:**
    ///   - `Tag(ProfilesService)`
    ///   - `Tag(ShowLoader)`
    #[derive(Default)]
    pub struct ProvideProfilesSchema {}

    #[async_trait]
    impl Provider<ProfilesSchema> for ProvideProfilesSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<ProfilesSchema>> {
            let service = i.get(&PROFILES_SERVICE).await?;
            let user_loader = i.get(&USER_LOADER).await?;

            let schema: ProfilesSchema = Schema::build(
                ProfilesQuery::default(),
                ProfilesMutation::default(),
                EmptySubscription,
            )
            .data(service)
            .data(DataLoader::new(user_loader, tokio::spawn))
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
