use std::sync::Arc;

use async_trait::async_trait;
use figment::{providers::Env, Figment};
use nakago::{config, hooks, Hook, Inject};
use serde::{Deserialize, Serialize};

use crate::auth;

/// The default Axum HTTP Config Loaders
pub fn default_loaders() -> Vec<Arc<dyn config::Loader>> {
    vec![
        Arc::<Loader>::default(),
        Arc::<auth::config::Loader>::default(),
    ]
}

/// Axum HTTP Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The port to bind to
    pub port: u16,

    /// The IP address to bind to, such as 0.0.0.0 or 127.0.0.1
    pub address: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 0,
            address: "0.0.0.0".to_string(),
        }
    }
}

/// The Axum HTTP Config Loader
#[derive(Default)]
pub struct Loader {}

impl config::Loader for Loader {
    fn load(&self, figment: Figment) -> Figment {
        // Split the HTTP variables
        figment
            .merge(Env::prefixed("HTTP").map(|key| key.as_str().replace("HTTP_", "HTTP.").into()))
    }
}

/// Add the default HTTP Config Loaders to the stack.
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
