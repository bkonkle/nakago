use std::sync::Arc;

use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;

use crate::{
    config::AppConfig,
    domains::{
        episodes::loaders::EPISODE_LOADER, episodes::service::EPISODES_SERVICE,
        profiles::loaders::PROFILE_LOADER, profiles::service::PROFILES_SERVICE,
        role_grants::loaders::ROLE_GRANT_LOADER, role_grants::service::ROLE_GRANTS_SERVICE,
        shows::loaders::SHOW_LOADER, shows::service::SHOWS_SERVICE, users::loaders::USER_LOADER,
        users::service::USERS_SERVICE,
    },
};
use crate::{
    domains::{
        episodes::resolver::{EpisodesMutation, EpisodesQuery},
        profiles::resolver::{ProfilesMutation, ProfilesQuery},
        shows::resolver::{ShowsMutation, ShowsQuery},
        users::resolver::{UsersMutation, UsersQuery},
    },
    utils::authz::OSO,
};

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, ProfilesQuery, ShowsQuery, EpisodesQuery);

/// The GraphQL top-level Mutation type
#[derive(MergedObject, Default)]
pub struct Mutation(
    UsersMutation,
    ProfilesMutation,
    ShowsMutation,
    EpisodesMutation,
);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Tag(GraphQLSchema)
pub const GRAPHQL_SCHEMA: Tag<GraphQLSchema> = Tag::new("GraphQLSchema");

/// Initialize all necessary dependencies to create a `GraphQLSchema`. Very simple dependency
/// injection based on async-graphql's `.data()` calls.
///
/// **Provides:** `GraphQLSchema`
///
/// **Depends on:**
///  - `Tag(AppConfig)`
///  - `Tag(Oso)`
///  - `Tag(UsersService)`
///  - `Tag(UserLoader)`
///  - `Tag(ProfilesService)`
///  - `Tag(ProfileLoader)`
///  - `Tag(RoleGrantsService)`
///  - `Tag(RoleGrantLoader)`
///  - `Tag(ShowsService)`
///  - `Tag(ShowLoader)`
///  - `Tag(EpisodesService)`
///  - `Tag(EpisodeLoader)`
#[derive(Default)]
pub struct ProvideGraphQLSchema {}

#[Provider]
#[async_trait]
impl Provider<GraphQLSchema> for ProvideGraphQLSchema {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<GraphQLSchema>> {
        let user_loader = i.get(&USER_LOADER).await?;
        let profile_loader = i.get(&PROFILE_LOADER).await?;
        let role_grant_loader = i.get(&ROLE_GRANT_LOADER).await?;
        let show_loader = i.get(&SHOW_LOADER).await?;
        let episode_loader = i.get(&EPISODE_LOADER).await?;
        let config = i.get_type::<AppConfig>().await?;
        let oso = i.get(&OSO).await?;
        let users = i.get(&USERS_SERVICE).await?;
        let profiles = i.get(&PROFILES_SERVICE).await?;
        let role_grants = i.get(&ROLE_GRANTS_SERVICE).await?;
        let shows = i.get(&SHOWS_SERVICE).await?;
        let episodes = i.get(&EPISODES_SERVICE).await?;

        Ok(Arc::new(
            Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                .data(config.clone())
                .data((*oso).clone())
                .data(users.clone())
                .data(user_loader.clone())
                .data(profile_loader.clone())
                .data(role_grant_loader.clone())
                .data(profiles.clone())
                .data(role_grants.clone())
                .data(shows.clone())
                .data(episodes.clone())
                .data(show_loader.clone())
                .data(episode_loader.clone())
                .finish(),
        ))
    }
}
