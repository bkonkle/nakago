use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Tag};

use crate::{
    config::AppConfig,
    domains::{
        episodes::providers::{EPISODES_SERVICE, EPISODE_LOADER},
        profiles::providers::{PROFILES_SERVICE, PROFILE_LOADER},
        role_grants::providers::{ROLE_GRANTS_SERVICE, ROLE_GRANT_LOADER},
        shows::providers::{SHOWS_SERVICE, SHOW_LOADER},
        users::providers::{USERS_SERVICE, USER_LOADER},
    },
};
use crate::{
    domains::{
        episodes::resolver::{EpisodesMutation, EpisodesQuery},
        profiles::resolver::{ProfilesMutation, ProfilesQuery},
        shows::resolver::{ShowsMutation, ShowsQuery},
        users::resolver::{UsersMutation, UsersQuery},
    },
    utils::providers::OSO,
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
pub struct InitGraphQLSchema {}

#[async_trait]
impl Hook for InitGraphQLSchema {
    async fn handle(&self, i: &Inject) -> InjectResult<()> {
        println!(">------ InitGraphQLSchema ------<");

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

        // Inject the initialized services into the `Schema` instance.
        i.inject(
            &GRAPHQL_SCHEMA,
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
        )
        .await?;

        Ok(())
    }
}
