use std::sync::Arc;

use figment::providers::Env;
use nakago::config;
use serde::{Deserialize, Serialize};

/// Return the default Config Loaders for SeaORM
pub fn default_config_loaders() -> Vec<Arc<dyn config::Loader>> {
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
    fn load_env(&self, env: Env) -> Env {
        // Split the Database variables
        env.map(|key| {
            key.as_str()
                .replace("DATABASE_POOL_", "DATABASE.POOL.")
                .into()
        })
        .map(|key| key.as_str().replace("DATABASE_", "DATABASE.").into())
    }
}
