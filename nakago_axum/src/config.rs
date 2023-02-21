use serde::{Deserialize, Serialize};

use crate::auth::config::AuthConfig;

/// HTTP Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpConfig {
    /// The port to bind to
    pub port: u16,

    /// The IP address to bind to, such as 0.0.0.0 or 127.0.0.1
    pub address: String,

    /// Auth config
    pub auth: AuthConfig,
}
