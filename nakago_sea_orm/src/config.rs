use std::sync::Arc;

use figment::{providers::Env, Figment};
use nakago::{Inject, Result, Tag};
use nakago_figment::{loaders, Loaders};
use serde::{Deserialize, Serialize};

/// Return the default Config Loaders for SeaORM
pub fn default_loaders() -> Vec<Arc<dyn nakago_figment::Loader>> {
    vec![Arc::<Loader>::default()]
}

/// Database Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Full database url
    pub url: String,

    /// Database debug logging
    pub debug: bool,

    /// Database pool config
    pub pool: DatabasePool,
}

/// Database pool config
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DatabasePool {
    /// Database pool min
    pub min: Option<i16>,

    /// Database pool max
    pub max: Option<i16>,
}

/// The Database Config Loader
#[derive(Default)]
pub struct Loader {}

impl nakago_figment::Loader for Loader {
    fn load(&self, figment: Figment) -> Figment {
        // Split the Database variables
        figment.merge(
            Env::prefixed("DATABASE")
                .map(|key| {
                    key.as_str()
                        .replace("DATABASE_POOL_", "DATABASE.POOL.")
                        .into()
                })
                .map(|key| key.as_str().replace("DATABASE_", "DATABASE.").into()),
        )
    }
}

/// Add the default HTTP Config Loaders to the stack.
pub async fn add_default_loaders(i: &Inject) -> Result<()> {
    loaders::Add::default().loaders(i, default_loaders()).await
}

/// Add the default HTTP Config Loaders to the stack.
pub async fn add_default_loaders_with_tag(i: &Inject, tag: &'static Tag<Loaders>) -> Result<()> {
    loaders::Add::default()
        .with_tag(tag)
        .loaders(i, default_loaders())
        .await
}
