use std::sync::Arc;

use figment::providers::Env;
use nakago::ConfigLoader;
use serde::{Deserialize, Serialize};

use crate::auth::config::AuthConfigLoader;

/// The default Axum HTTP Config Loaders
pub fn default_http_config_loaders() -> Vec<Arc<dyn nakago::config::loader::ConfigLoader>> {
    vec![
        Arc::<HttpConfigLoader>::default(),
        Arc::<AuthConfigLoader>::default(),
    ]
}

/// HTTP Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpConfig {
    /// The port to bind to
    pub port: u16,

    /// The IP address to bind to, such as 0.0.0.0 or 127.0.0.1
    pub address: String,
}

impl Default for HttpConfig {
    fn default() -> Self {
        HttpConfig {
            port: 0,
            address: "0.0.0.0".to_string(),
        }
    }
}

/// The Axum HTTP Config Loader
#[derive(Default)]
pub struct HttpConfigLoader {}

impl ConfigLoader for HttpConfigLoader {
    fn load_env(&self, env: Env) -> Env {
        // Split the HTTP variables
        env.map(|key| key.as_str().replace("HTTP_", "HTTP.").into())
    }
}
