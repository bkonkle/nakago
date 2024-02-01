use nakago::{inject, EventType};
use nakago_axum::{
    auth::{jwks, validator, Validator, JWKS},
    AxumApplication,
};
use nakago_ws::{connections, handler};

use crate::{
    authz::{self, ProvideOso, OSO},
    config::{Config, CONFIG},
    domains::graphql,
    http::{
        self,
        events::{self, CONNECTIONS, CONTROLLER},
    },
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    // Dependencies

    app.provide(&JWKS, jwks::Provide::new(Some(&CONFIG)))
        .await?;

    app.provide_type::<Validator>(validator::Provide::default())
        .await?;

    app.provide(
        &nakago_sea_orm::CONNECTION,
        nakago_sea_orm::connection::Provide::new(Some(&CONFIG)),
    )
    .await?;

    app.provide(&OSO, ProvideOso::default()).await?;

    app.provide(&events::CONNECTIONS, connections::Provide::default())
        .await?;

    app.provide(
        &events::HANDLER,
        handler::Provide::new(Some(&CONNECTIONS), Some(&CONTROLLER)),
    )
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
