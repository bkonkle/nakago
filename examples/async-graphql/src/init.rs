use std::path::PathBuf;

use nakago::{self, Inject};
use nakago_axum::auth::{jwks, validator, Jwks, Validator};
use oso::Oso;
use sea_orm::DatabaseConnection;

use crate::{
    authz::{self, ProvideOso},
    domains::graphql,
    http, Config,
};

/// Create a dependency injection container for the top-level application
pub async fn app(config_path: Option<PathBuf>) -> nakago::Result<Inject> {
    let i = Inject::default();

    i.provide::<Jwks>(jwks::Provide::<Config>::default())
        .await?;

    i.provide::<Validator>(validator::Provide::default())
        .await?;

    i.provide::<DatabaseConnection>(nakago_sea_orm::connection::Provide::<Config>::new())
        .await?;

    i.provide::<Oso>(ProvideOso::default()).await?;

    // Add config loaders before the Config is initialized
    nakago_axum::config::add_default_loaders(&i).await?;
    nakago_sea_orm::config::add_default_loaders(&i).await?;

    // Initialize the Config
    nakago_figment::Init::<Config>::default()
        .maybe_with_path(config_path)
        .init(&i)
        .await?;

    // Load phase
    authz::load(&i).await?;
    http::router::load(&i).await?;
    graphql::load(&i).await?;

    // Init phase
    graphql::init(&i).await?;

    Ok(i)
}
