use nakago::{inject, EventType};
use nakago_axum::{
    auth::{jwks, validator, Jwks, Validator},
    AxumApplication,
};
use oso::Oso;
use sea_orm::DatabaseConnection;

use crate::{
    authz::{self, ProvideOso},
    domains::graphql,
    http, Config,
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default();

    // Dependencies

    app.provide::<Jwks>(jwks::Provide::<Config>::new(None))
        .await?;

    app.provide::<Validator>(validator::Provide::default())
        .await?;

    app.provide::<DatabaseConnection>(nakago_sea_orm::connection::Provide::<Config>::new(None))
        .await?;

    app.provide::<Oso>(ProvideOso::default()).await?;

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
