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
        episodes::schema::LoadEpisodes, profiles::schema::LoadProfiles,
        role_grants::schema::LoadRoleGrants, shows::schema::LoadShows, users::schema::LoadUsers,
    },
    events::{
        ProvideConnections, ProvideSocket, {CONNECTIONS, SOCKET_HANDLER},
    },
    graphql::{InitGraphQL, GRAPHQL_SCHEMA, GRAPHQL_SCHEMA_BUILDER},
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

    // GraphQL

    app.on(&EventType::Init, InitGraphQL::default());

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

        // Handle some sub-hooks to load more dependencies

        i.handle(LoadUsers::default()).await?;

        i.handle(LoadRoleGrants::default()).await?;

        i.handle(LoadProfiles::default()).await?;

        i.handle(LoadShows::default()).await?;

        i.handle(LoadEpisodes::default()).await?;

        i.handle(LoadAuthz::default()).await?;

        Ok(())
    }
}
