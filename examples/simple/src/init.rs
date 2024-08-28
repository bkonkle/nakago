use std::path::PathBuf;

use nakago::{Inject, Result};
use nakago_axum::{
    auth::{jwks, validator, Jwks, Validator},
    config,
};

use crate::config::Config;

/// Create a dependency injection container for the top-level application
pub async fn app(config_path: Option<PathBuf>) -> Result<Inject> {
    let i = Inject::default();

    i.provide::<Jwks>(jwks::Provide::<Config>::default())
        .await?;

    i.provide::<Validator>(validator::Provide::default())
        .await?;

    // Add config loaders before the Config is initialized
    config::add_default_loaders(&i).await?;

    // Initialize the Config
    nakago_figment::Init::<Config>::default()
        .maybe_with_path(config_path)
        .init(&i)
        .await?;

    Ok(i)
}
