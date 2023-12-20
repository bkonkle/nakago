use std::ops::Deref;

use anyhow::Result;
use nakago_warp::auth::{validator, Validator};

use nakago_examples_simple_warp::{init, Config};

pub struct TestUtils(nakago_warp::test::Utils<Config>);

impl Deref for TestUtils {
    type Target = nakago_warp::test::Utils<Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TestUtils {
    pub async fn init() -> Result<Self> {
        let app = init::app().await?;

        app.replace_type_with::<Validator>(validator::ProvideUnverified::default())
            .await?;

        let config_path = std::env::var("CONFIG_PATH_SIMPLE_WARP")
            .unwrap_or_else(|_| "examples/simple-warp/config/test.toml".to_string());

        let utils = nakago_warp::test::Utils::init(app, &config_path, "/").await?;

        Ok(Self(utils))
    }
}
