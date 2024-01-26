use std::sync::Arc;

use async_trait::async_trait;
use figment::{providers::Env, Figment};
use nakago::{
    config::{self},
    hooks, Hook, Inject,
};
use serde::{Deserialize, Serialize};

/// Return the default Config Loaders for SeaORM
pub fn default_loaders() -> Vec<Arc<dyn config::Loader>> {
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

impl config::Loader for Loader {
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

/// Add the default SeaOrm Config Loaders to the stack.
pub struct AddLoaders(config::AddLoaders);

impl Default for AddLoaders {
    fn default() -> Self {
        Self(config::AddLoaders::new(default_loaders()))
    }
}

#[async_trait]
impl Hook for AddLoaders {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        self.0.handle(i).await
    }
}
