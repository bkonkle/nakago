use figment::providers::Env;
use nakago::config;
use serde::{Deserialize, Serialize};

/// Auth config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// OAuth2 url
    pub url: String,

    /// OAuth2 audience
    pub audience: String,

    /// Auth client config
    pub client: Client,
}

/// Auth client config
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Client {
    /// OAuth2 client id
    pub id: Option<String>,

    /// OAuth2 client secret
    pub secret: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "https://".to_string(),
            audience: "localhost".to_string(),
            client: Client::default(),
        }
    }
}

/// The Auth Config Loader
#[derive(Default)]
pub struct Loader {}

impl config::Loader for Loader {
    fn load_env(&self, env: Env) -> Env {
        // Split the Auth variables
        env.map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
            .map(|key| key.as_str().replace("AUTH_", "AUTH.").into())
    }
}
