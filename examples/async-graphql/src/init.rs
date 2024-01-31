use nakago::{inject, EventType};
use nakago_axum::{
    auth::{jwks, validator, Validator, JWKS},
    AxumApplication,
};
use nakago_ws::socket;

use crate::{
    authz::{self, ProvideOso, OSO},
    config::{Config, CONFIG},
    domains::graphql,
    http,
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    // Dependencies

    app.provide(&JWKS, jwks::Provide::default().with_config_tag(&CONFIG))
        .await?;

    app.provide_type::<Validator>(validator::Provide::default())
        .await?;

    app.provide(
        &nakago_sea_orm::CONNECTION,
        nakago_sea_orm::connection::Provide::default().with_config_tag(&CONFIG),
    )
    .await?;

    app.provide(&OSO, ProvideOso::default()).await?;

    app.provide(
        &events::CONNECTIONS,
        events::connections::Provide::default(),
    )
    .await?;

    app.provide(&socket::HANDLER, socket::Provide::default())
        .await?;

    // Loading

    app.on(&EventType::Load, nakago_axum::config::AddLoaders::default());

    app.on(
        &EventType::Load,
        nakago_sea_orm::config::AddLoaders::default(),
    );

    app.on(&EventType::Load, authz::Load::default());
    app.on(&EventType::Load, graphql::Load::default());
    app.on(&EventType::Load, http::Load::default());

    // Initialization

    app.on(&EventType::Init, graphql::Init::default());
    app.on(&EventType::Init, http::Init::default());

    Ok(app)
}
