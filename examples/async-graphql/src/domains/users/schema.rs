use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use crate::{domains::profiles::service::PROFILES_SERVICE, graphql::GRAPHQL_SCHEMA_BUILDER};

use super::{
    loaders::{ProvideUserLoader, USER_LOADER},
    service::{ProvideUsersService, USERS_SERVICE},
};

/// Provide dependencies needed for the Users domain
#[derive(Default)]
pub struct LoadUsers {}

#[async_trait]
impl Hook for LoadUsers {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&USERS_SERVICE, ProvideUsersService::default())
            .await?;

        i.provide(&USER_LOADER, ProvideUserLoader::default())
            .await?;

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
        let service = i.get(&USERS_SERVICE).await?;
        let profiles = i.get(&PROFILES_SERVICE).await?;

        i.modify(&GRAPHQL_SCHEMA_BUILDER, |builder| {
            Ok(builder.data(service.clone()).data(profiles.clone()))
        })
        .await?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use async_graphql::{EmptySubscription, Schema};
    use nakago::{Provider, Tag};

    use crate::domains::users::resolver::{UsersMutation, UsersQuery};

    use super::*;

    /// Tag(UsersSchema)
    #[allow(dead_code)]
    pub const USERS_SCHEMA: Tag<Box<UsersSchema>> = Tag::new("UsersSchema");

    /// The UsersSchema, covering just the Users domain. Useful for testing in isolation.
    pub type UsersSchema = Schema<UsersQuery, UsersMutation, EmptySubscription>;

    /// Provide the UsersSchema
    ///
    /// **Provides:** `Arc<UsersSchema>`
    ///
    /// **Depends on:**
    ///   - `Tag(UsersService)`
    ///   - `Tag(ProfilesService)`
    #[derive(Default)]
    pub struct ProvideUsersSchema {}

    #[async_trait]
    impl Provider<UsersSchema> for ProvideUsersSchema {
        async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<UsersSchema>> {
            let service = i.get(&USERS_SERVICE).await?;
            let profiles = i.get(&PROFILES_SERVICE).await?;

            let schema: UsersSchema = Schema::build(
                UsersQuery::default(),
                UsersMutation::default(),
                EmptySubscription,
            )
            .data(service)
            .data(profiles)
            .finish();

            Ok(Arc::new(schema))
        }
    }
}
