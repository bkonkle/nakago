use async_trait::async_trait;
use nakago::{config::AddConfigLoaders, EventType, Hook, Inject, InjectResult};
use nakago_axum::{
    auth::{
        ProvideAuthState, ProvideJwks, {AUTH_STATE, JWKS},
    },
    AxumApplication, InitRoute,
};
use nakago_sea_orm::{ProvideConnection, DATABASE_CONNECTION};

use crate::{
    config::{AppConfig, CONFIG},
    routes::{new_health_route, new_user_route, AppState, ProvideAppState, STATE},
};

/// Create a default AxumApplication instance
pub fn app() -> AxumApplication<AppConfig, AppState> {
    let mut app = AxumApplication::default()
        .with_config_tag(&CONFIG)
        .with_state_tag(&STATE);

    // Config

    app.on(
        &EventType::Load,
        AddConfigLoaders::new(nakago_sea_orm::default_config_loaders()),
    );

    // Dependencies

    app.on(&EventType::Load, Load::default());

    // Routes

    app.on(&EventType::Init, InitRoute::new(new_health_route));
    app.on(&EventType::Init, InitRoute::new(new_user_route));

    app
}

/// Provides default dependencies for the Application
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&JWKS, ProvideJwks::<AppConfig>::default())
            .await?;

        i.provide(
            &DATABASE_CONNECTION,
            ProvideConnection::<AppConfig>::default(),
        )
        .await?;

        i.provide(&AUTH_STATE, ProvideAuthState::default()).await?;

        i.provide(&STATE, ProvideAppState::default()).await?;

        Ok(())
    }
}
