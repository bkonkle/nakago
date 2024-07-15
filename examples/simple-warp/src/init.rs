use nakago::{inject, EventType};
use nakago_warp::{
    auth::{jwks, validator, Jwks, Validator},
    config, WarpApplication,
};

use crate::{
    config::{Config, CONFIG},
    http,
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<WarpApplication<Config>> {
    let mut app = WarpApplication::default().with_config_tag(&CONFIG);

    // Dependencies

    app.provide::<Jwks>(jwks::Provide::default().with_config_tag(&CONFIG))
        .await?;

    app.provide::<Validator>(validator::Provide::default())
        .await?;

    // Loading

    app.on(&EventType::Load, config::AddLoaders::default());

    // Initialization

    app.on(&EventType::Init, http::Init::default());

    Ok(app)
}
