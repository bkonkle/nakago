use hyper::Method;
use nakago::{config, inject, EventType};
use nakago_async_graphql::schema;
use nakago_axum::{
    auth::{self, jwks, Validator, JWKS},
    routes, AxumApplication,
};
use nakago_sea_orm::{connection, CONNECTION};

use crate::{
    config::{Config, CONFIG},
    domains::{episodes, profiles, role_grants, shows, users},
    events::{socket, ProvideConnections, CONNECTIONS},
    graphql, http,
    utils::authz::{self, ProvideOso, OSO},
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    // Dependencies

    app.provide(&JWKS, jwks::Provide::default().with_config_tag(&CONFIG))
        .await?;

    app.provide_type::<Validator>(auth::subject::Provide::default())
        .await?;

    app.provide(
        &CONNECTION,
        connection::Provide::default().with_config_tag(&CONFIG),
    )
    .await?;

    app.provide(&OSO, ProvideOso::default()).await?;

    app.provide(&CONNECTIONS, ProvideConnections::default())
        .await?;

    app.provide(&socket::HANDLER, socket::Provide::default())
        .await?;

    app.provide(&graphql::SCHEMA_BUILDER, schema::ProvideBuilder::default())
        .await?;

    // Loading

    app.on(
        &EventType::Load,
        config::AddLoaders::new(nakago_sea_orm::default_config_loaders()),
    );

    app.on(&EventType::Load, users::schema::Load::default());

    app.on(&EventType::Load, role_grants::schema::Load::default());

    app.on(&EventType::Load, profiles::schema::Load::default());

    app.on(&EventType::Load, shows::schema::Load::default());

    app.on(&EventType::Load, episodes::schema::Load::default());

    app.on(&EventType::Load, authz::Load::default());

    // Initialization

    app.on(&EventType::Init, graphql::Init::default());

    app.on(
        &EventType::Init,
        schema::Init::default()
            .with_builder_tag(&graphql::SCHEMA_BUILDER)
            .with_schema_tag(&graphql::SCHEMA),
    );

    app.on(
        &EventType::Init,
        routes::Init::new(Method::GET, "/health", http::health::health_check),
    );

    app.on(
        &EventType::Init,
        routes::Init::new(Method::GET, "/graphql", http::graphql::graphiql),
    );

    app.on(
        &EventType::Init,
        routes::Init::new(Method::POST, "/graphql", http::graphql::resolve),
    );

    app.on(
        &EventType::Init,
        routes::Init::new(Method::GET, "/events", http::events::upgrade),
    );

    Ok(app)
}
