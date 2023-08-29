use async_trait::async_trait;
use nakago::{config::AddConfigLoaders, EventType, Hook, Inject, InjectResult};
use nakago_async_graphql::schema::{InitSchema, SchemaBuilderProvider};
use nakago_axum::{
    auth::{
        ProvideAuthState, ProvideJwks, {AUTH_STATE, JWKS},
    },
    AxumApplication, InitRoute,
};
use nakago_sea_orm::{ProvideConnection, DATABASE_CONNECTION};

use crate::{
    config::AppConfig,
    domains::{
        episodes::{
            loaders::{ProvideEpisodeLoader, EPISODE_LOADER},
            schema::InitGraphQLEpisodes,
            service::{ProvideEpisodesService, EPISODES_SERVICE},
        },
        profiles::{
            loaders::{ProvideProfileLoader, PROFILE_LOADER},
            schema::InitGraphQLProfiles,
            service::{ProvideProfilesService, PROFILES_SERVICE},
        },
        role_grants::{
            loaders::{ProvideRoleGrantLoader, ROLE_GRANT_LOADER},
            service::{ProvideRoleGrantsService, ROLE_GRANTS_SERVICE},
        },
        shows::{
            loaders::{ProvideShowLoader, SHOW_LOADER},
            schema::InitGraphQLShows,
            service::{ProvideShowsService, SHOWS_SERVICE},
        },
        users::{
            loaders::{ProvideUserLoader, USER_LOADER},
            schema::InitGraphQLUsers,
            service::{ProvideUsersService, USERS_SERVICE},
        },
    },
    events::{
        ProvideConnections, ProvideSocket, {CONNECTIONS, SOCKET_HANDLER},
    },
    graphql::{GRAPHQL_SCHEMA, GRAPHQL_SCHEMA_BUILDER},
    routes::{new_events_route, new_graphql_route, new_health_route, AppState, ProvideAppState},
    utils::authz::{LoadAuthz, ProvideOso, OSO},
};

/// Create a default AxumApplication instance
pub fn app() -> AxumApplication<AppConfig> {
    let mut app = AxumApplication::<AppConfig>::default();

    // Config

    app.on(
        &EventType::Load,
        AddConfigLoaders::new(nakago_sea_orm::default_config_loaders()),
    );

    // Dependencies

    app.on(&EventType::Load, Load::default());
    app.on(&EventType::Load, LoadAuthz::default());

    // GraphQL

    app.on(&EventType::Init, InitGraphQLUsers::default());
    app.on(&EventType::Init, InitGraphQLProfiles::default());
    app.on(&EventType::Init, InitGraphQLShows::default());
    app.on(&EventType::Init, InitGraphQLEpisodes::default());

    app.on(
        &EventType::Init,
        InitSchema::default()
            .with_builder_tag(&GRAPHQL_SCHEMA_BUILDER)
            .with_schema_tag(&GRAPHQL_SCHEMA),
    );

    // Routes

    app.on(&EventType::Init, InitRoute::new(new_health_route));
    app.on(&EventType::Init, InitRoute::new(new_graphql_route));
    app.on(&EventType::Init, InitRoute::new(new_events_route));

    app
}

/// Provides default dependencies for the Application
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&USERS_SERVICE, ProvideUsersService::default())
            .await?;

        i.provide(&USER_LOADER, ProvideUserLoader::default())
            .await?;

        i.provide(&PROFILES_SERVICE, ProvideProfilesService::default())
            .await?;

        i.provide(&PROFILE_LOADER, ProvideProfileLoader::default())
            .await?;

        i.provide(&ROLE_GRANTS_SERVICE, ProvideRoleGrantsService::default())
            .await?;

        i.provide(&ROLE_GRANT_LOADER, ProvideRoleGrantLoader::default())
            .await?;

        i.provide(&SHOWS_SERVICE, ProvideShowsService::default())
            .await?;

        i.provide(&SHOW_LOADER, ProvideShowLoader::default())
            .await?;

        i.provide(&EPISODES_SERVICE, ProvideEpisodesService::default())
            .await?;

        i.provide(&EPISODE_LOADER, ProvideEpisodeLoader::default())
            .await?;

        i.provide(&JWKS, ProvideJwks::<AppConfig>::default())
            .await?;

        i.provide(
            &DATABASE_CONNECTION,
            ProvideConnection::<AppConfig>::default(),
        )
        .await?;

        i.provide(&OSO, ProvideOso::default()).await?;

        i.provide(&CONNECTIONS, ProvideConnections::default())
            .await?;

        i.provide(&SOCKET_HANDLER, ProvideSocket::default()).await?;

        i.provide(&AUTH_STATE, ProvideAuthState::default()).await?;

        i.provide(&GRAPHQL_SCHEMA_BUILDER, SchemaBuilderProvider::default())
            .await?;

        i.provide_type::<AppState>(ProvideAppState::default())
            .await?;

        Ok(())
    }
}
