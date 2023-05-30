use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_trait::async_trait;
use nakago::inject;

use crate::{
    config::AppConfig,
    domains::{
        episodes::{EPISODES_SERVICE, EPISODE_LOADER},
        profiles::{PROFILES_SERVICE, PROFILE_LOADER},
        role_grants::{ROLE_GRANTS_SERVICE, ROLE_GRANT_LOADER},
        shows::{SHOWS_SERVICE, SHOW_LOADER},
        users::{USERS_SERVICE, USER_LOADER},
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

pub const GRAPHQL_SCHEMA: inject::Tag<GraphQLSchema> = inject::Tag::new("GraphQLSchema");

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
impl inject::Hook for InitGraphQLSchema {
    async fn handle(&self, i: &mut inject::Inject) -> inject::Result<()> {
        let user_loader = i.consume(&USER_LOADER)?;
        let profile_loader = i.consume(&PROFILE_LOADER)?;
        let role_grant_loader = i.consume(&ROLE_GRANT_LOADER)?;
        let show_loader = i.consume(&SHOW_LOADER)?;
        let episode_loader = i.consume(&EPISODE_LOADER)?;
        let config = i.get_type::<AppConfig>()?;
        let oso = i.get(&OSO)?;
        let users = i.get(&USERS_SERVICE)?;
        let profiles = i.get(&PROFILES_SERVICE)?;
        let role_grants = i.get(&ROLE_GRANTS_SERVICE)?;
        let shows = i.get(&SHOWS_SERVICE)?;
        let episodes = i.get(&EPISODES_SERVICE)?;

        // Inject the initialized services into the `Schema` instance.
        i.inject(
            &GRAPHQL_SCHEMA,
            Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                .data(config.clone())
                .data(oso.clone())
                .data(users.clone())
                .data(user_loader)
                .data(profile_loader)
                .data(role_grant_loader)
                .data(profiles.clone())
                .data(role_grants.clone())
                .data(shows.clone())
                .data(episodes.clone())
                .data(show_loader)
                .data(episode_loader)
                .finish(),
        )?;

        Ok(())
    }
}
