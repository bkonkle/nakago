use nakago::{config, inject, EventType};
use nakago_axum::{
    auth::{self, jwks, Validator, JWKS},
    config::default_loaders,
    AxumApplication,
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

    app.provide_type::<Validator>(auth::subject::Provide::default())
        .await?;

    // Config

    app.on(&EventType::Load, config::AddLoaders::new(default_loaders()));

    // Routes

    app.on(&EventType::Init, http::Init::default());

    Ok(app)
}
