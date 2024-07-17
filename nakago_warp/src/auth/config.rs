use figment::{providers::Env, Figment};
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

impl nakago_figment::Loader for Loader {
    fn load(&self, figment: Figment) -> Figment {
        // Split the Auth variables
        figment.merge(
            Env::prefixed("AUTH")
                .map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
                .map(|key| key.as_str().replace("AUTH_", "AUTH.").into()),
        )
    }
}
