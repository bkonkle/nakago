use std::ops::Deref;

use anyhow::Result;
use nakago_axum::auth::{validator, Validator};

use nakago_examples_simple::{init, Config};

pub struct TestUtils(nakago_axum::test::Utils<Config>);

impl Deref for TestUtils {
    type Target = nakago_axum::test::Utils<Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TestUtils {
    pub async fn init() -> Result<Self> {
        let app = init::app().await?;

        app.replace_type_with::<Validator>(validator::ProvideUnverified::default())
            .await?;

        let config_path = std::env::var("CONFIG_PATH_SIMPLE")
            .unwrap_or_else(|_| "examples/simple/config/test.toml".to_string());

        let utils = nakago_axum::test::Utils::init(app, &config_path, "/").await?;

        Ok(Self(utils))
    }
}
