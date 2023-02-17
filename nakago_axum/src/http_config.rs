use figment::providers::Env;
use serde::{Deserialize, Serialize};

use nakago::config::ConfigLoader;

/// HTTP Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpConfig {
    /// The port to bind to
    pub port: u16,

    /// The IP address to bind to, such as 0.0.0.0 or 127.0.0.1
    pub address: String,

    /// Auth config
    pub auth: Auth,
}

/// Auth config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    /// OAuth2 url
    pub url: String,

    /// OAuth2 audience
    pub audience: String,

    /// Auth client config
    pub client: AuthClient,
}

/// Auth client config
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AuthClient {
    /// OAuth2 client id
    pub id: Option<String>,

    /// OAuth2 client secret
    pub secret: Option<String>,
}

impl ConfigLoader for HttpConfig {
    fn load_env(&self, env: Env) -> Env {
        // Split the Auth variables
        env.map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
            .map(|key| key.as_str().replace("AUTH_", "AUTH.").into())
    }
}
