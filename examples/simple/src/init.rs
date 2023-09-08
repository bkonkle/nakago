use nakago::{config::AddConfigLoaders, EventType, InjectResult};
use nakago_axum::{
    auth::{ProvideAuthState, ProvideJwks, AUTH_STATE, JWKS},
    config::default_http_config_loaders,
    AxumApplication, InitRoute,
};

use crate::{
    config::{AppConfig, CONFIG},
    http::{
        routes::{new_health_route, new_user_route},
        state::{AppState, ProvideAppState, STATE},
    },
};

/// Create a default AxumApplication instance
pub async fn app() -> InjectResult<AxumApplication<AppConfig, AppState>> {
    let mut app = AxumApplication::default()
        .with_config_tag(&CONFIG)
        .with_state_tag(&STATE);

    // Dependencies

    app.provide(&JWKS, ProvideJwks::default().with_config_tag(&CONFIG))
        .await?;

    app.provide(&AUTH_STATE, ProvideAuthState::default())
        .await?;

    app.provide(&STATE, ProvideAppState::default()).await?;

    // Config

    app.on(
        &EventType::Load,
        AddConfigLoaders::new(default_http_config_loaders()),
    );

    // Routes

    app.on(&EventType::Init, InitRoute::new(new_health_route));
    app.on(&EventType::Init, InitRoute::new(new_user_route));

    Ok(app)
}
