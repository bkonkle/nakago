use figment::providers::Env;
use nakago::config::loader::ConfigLoader;
use serde::{Deserialize, Serialize};

/// Auth config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    /// OAuth2 url
    pub url: String,

    /// OAuth2 audience
    pub audience: String,

    /// Auth client config
    pub client: AuthClientConfig,
}

/// Auth client config
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AuthClientConfig {
    /// OAuth2 client id
    pub id: Option<String>,

    /// OAuth2 client secret
    pub secret: Option<String>,
}

/// The Auth Config Loader
#[derive(Default)]
pub struct AuthConfigLoader {}

impl ConfigLoader for AuthConfigLoader {
    fn load_env(&self, env: Env) -> Env {
        // Split the Auth variables
        env.map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
            .map(|key| key.as_str().replace("AUTH_", "AUTH.").into())
    }
}
