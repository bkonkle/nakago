use nakago::{inject, EventType};
use nakago_axum::{
    auth::{jwks, validator, Validator, JWKS},
    config, AxumApplication,
};

use crate::{
    config::{Config, CONFIG},
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

    // Loading

    app.on(&EventType::Load, config::AddLoaders::default());

    // Initialization

    app.on(&EventType::Init, http::Init::default());

    Ok(app)
}
