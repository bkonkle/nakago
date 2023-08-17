use std::sync::Arc;

use figment::providers::Env;
use nakago::config::{loader::ConfigLoader, AddConfigLoaders};
use serde::{Deserialize, Serialize};

/// Database Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Database hostname/IP
    pub hostname: String,

    /// Database username
    pub username: String,

    /// Database password
    pub password: String,

    /// Database name
    pub name: String,

    /// Database port
    pub port: u16,

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
pub struct DatabaseConfigLoader {}

impl ConfigLoader for DatabaseConfigLoader {
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

/// Add the Database Config Loader to the Application's registered loaders
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
pub fn init_config_loaders() -> AddConfigLoaders {
    AddConfigLoaders::new(vec![Arc::<DatabaseConfigLoader>::default()])
}
