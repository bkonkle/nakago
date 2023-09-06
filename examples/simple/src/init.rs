use async_trait::async_trait;
use nakago::{EventType, Hook, Inject, InjectResult};
use nakago_axum::{AxumApplication, InitRoute};

use crate::{
    config::{AppConfig, CONFIG},
    http::{
        routes::new_health_route,
        state::{AppState, ProvideAppState, STATE},
    },
};

/// Create a default AxumApplication instance
pub fn app() -> AxumApplication<AppConfig, AppState> {
    let mut app = AxumApplication::default()
        .with_config_tag(&CONFIG)
        .with_state_tag(&STATE);

    // Dependencies

    app.on(&EventType::Load, Load::default());

    // Routes

    app.on(&EventType::Init, InitRoute::new(new_health_route));

    app
}

/// Provides default dependencies for the Application
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.provide(&STATE, ProvideAppState::default()).await?;

        Ok(())
    }
}
