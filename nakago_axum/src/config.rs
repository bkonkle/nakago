use std::sync::Arc;

use nakago::config::AddConfigLoaders;
use serde::{Deserialize, Serialize};

use crate::auth::config::AuthConfigLoader;

/// HTTP Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpConfig {
    /// The port to bind to
    pub port: u16,

    /// The IP address to bind to, such as 0.0.0.0 or 127.0.0.1
    pub address: String,
}

/// Add the Config Loaders that are custom to this app
pub fn add_http_config_loaders() -> AddConfigLoaders {
    AddConfigLoaders::new(vec![Arc::<AuthConfigLoader>::default()])
}
