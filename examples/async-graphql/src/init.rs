use nakago::{config::AddConfigLoaders, EventType, InjectResult};
use nakago_async_graphql::schema::{InitSchema, SchemaBuilderProvider};
use nakago_axum::{
    auth::{
        ProvideAuthState, ProvideJwks, {AUTH_STATE, JWKS},
    },
    AxumApplication, InitRoute,
};
use nakago_sea_orm::{ProvideConnection, DATABASE_CONNECTION};

use crate::{
    config::{AppConfig, CONFIG},
    domains::{
        episodes::schema::LoadEpisodes, profiles::schema::LoadProfiles,
        role_grants::schema::LoadRoleGrants, shows::schema::LoadShows, users::schema::LoadUsers,
    },
    events::{
        ProvideConnections, ProvideSocket, {CONNECTIONS, SOCKET_HANDLER},
    },
    graphql::{InitGraphQL, GRAPHQL_SCHEMA, GRAPHQL_SCHEMA_BUILDER},
    http::{
        routes::{new_events_route, new_graphql_route, new_health_route},
        state::{AppState, ProvideAppState, STATE},
    },
    utils::authz::{LoadAuthz, ProvideOso, OSO},
};

/// Create a default AxumApplication instance
pub async fn app() -> InjectResult<AxumApplication<AppConfig, AppState>> {
    let mut app = AxumApplication::default()
        .with_config_tag(&CONFIG)
        .with_state_tag(&STATE);

    // Dependencies

    app.provide(&JWKS, ProvideJwks::default().with_config_tag(&CONFIG))
        .await?;

    app.provide(
        &DATABASE_CONNECTION,
        ProvideConnection::default().with_config_tag(&CONFIG),
    )
    .await?;

    app.provide(&OSO, ProvideOso::default()).await?;

    app.provide(&CONNECTIONS, ProvideConnections::default())
        .await?;

    app.provide(&SOCKET_HANDLER, ProvideSocket::default())
        .await?;

    app.provide(&GRAPHQL_SCHEMA_BUILDER, SchemaBuilderProvider::default())
        .await?;

    app.provide(&AUTH_STATE, ProvideAuthState::default())
        .await?;

    app.provide(&STATE, ProvideAppState::default()).await?;

    // Loading

    app.on(
        &EventType::Load,
        AddConfigLoaders::new(nakago_sea_orm::default_config_loaders()),
    );

    app.on(&EventType::Load, LoadUsers::default());

    app.on(&EventType::Load, LoadRoleGrants::default());

    app.on(&EventType::Load, LoadProfiles::default());

    app.on(&EventType::Load, LoadShows::default());

    app.on(&EventType::Load, LoadEpisodes::default());

    app.on(&EventType::Load, LoadAuthz::default());

    // Initialization

    app.on(&EventType::Init, InitGraphQL::default());

    app.on(
        &EventType::Init,
        InitSchema::default()
            .with_builder_tag(&GRAPHQL_SCHEMA_BUILDER)
            .with_schema_tag(&GRAPHQL_SCHEMA),
    );

    app.on(&EventType::Init, InitRoute::new(new_health_route));
    app.on(&EventType::Init, InitRoute::new(new_graphql_route));
    app.on(&EventType::Init, InitRoute::new(new_events_route));

    Ok(app)
}
