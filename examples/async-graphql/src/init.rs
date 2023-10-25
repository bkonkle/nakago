use nakago::{config, inject, EventType};
use nakago_async_graphql::schema;
use nakago_axum::{
    auth::{self, jwks, JWKS},
    routes, AxumApplication,
};
use nakago_sea_orm::{connection, CONNECTION};

use crate::{
    config::{Config, CONFIG},
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
        state::{self, State, STATE},
    },
    utils::authz::{LoadAuthz, ProvideOso, OSO},
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config, State>> {
    let mut app = AxumApplication::default()
        .with_config_tag(&CONFIG)
        .with_state_tag(&STATE);

    // Dependencies

    app.provide(&JWKS, jwks::Provide::default().with_config_tag(&CONFIG))
        .await?;

    app.provide(
        &CONNECTION,
        connection::Provide::default().with_config_tag(&CONFIG),
    )
    .await?;

    app.provide(&OSO, ProvideOso::default()).await?;

    app.provide(&CONNECTIONS, ProvideConnections::default())
        .await?;

    app.provide(&SOCKET_HANDLER, ProvideSocket::default())
        .await?;

    app.provide(&GRAPHQL_SCHEMA_BUILDER, schema::ProvideBuilder::default())
        .await?;

    app.provide(&auth::STATE, auth::state::Provide::default())
        .await?;

    app.provide(&STATE, state::Provide::default()).await?;

    // Loading

    app.on(
        &EventType::Load,
        config::AddLoaders::new(nakago_sea_orm::default_config_loaders()),
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
        schema::Init::default()
            .with_builder_tag(&GRAPHQL_SCHEMA_BUILDER)
            .with_schema_tag(&GRAPHQL_SCHEMA),
    );

    app.on(&EventType::Init, routes::Init::new(new_health_route));
    app.on(&EventType::Init, routes::Init::new(new_graphql_route));
    app.on(&EventType::Init, routes::Init::new(new_events_route));

    Ok(app)
}
