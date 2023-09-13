use std::ops::Deref;

use anyhow::Result;
use nakago_axum::auth::{authenticate::ProvideUnverifiedAuthState, AUTH_STATE};

use nakago_examples_simple::{config::AppConfig, http::state::AppState, init};

pub struct TestUtils(nakago_axum::test::utils::TestUtils<AppConfig, AppState>);

impl Deref for TestUtils {
    type Target = nakago_axum::test::utils::TestUtils<AppConfig, AppState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TestUtils {
    pub async fn init() -> Result<Self> {
        let app = init::app().await?;

        app.replace_with(&AUTH_STATE, ProvideUnverifiedAuthState::default())
            .await?;

        let config_path = std::env::var("CONFIG_PATH_SIMPLE")
            .unwrap_or_else(|_| "examples/simple/config/test.toml".to_string());

        let utils = nakago_axum::test::utils::TestUtils::init(app, &config_path, "/").await?;

        Ok(Self(utils))
    }
}
