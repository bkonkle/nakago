use std::sync::Arc;

use figment::{providers::Env, Figment};
use nakago::{Inject, Tag};
use nakago_figment::{loaders, Loaders};
use serde::{Deserialize, Serialize};

/// The default Warp HTTP Config Loaders
pub fn default_loaders() -> Vec<Arc<dyn nakago_figment::Loader>> {
    vec![Arc::<Loader>::default()]
}

/// Warp HTTP Config
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

/// The Warp HTTP Config Loader
#[derive(Default)]
pub struct Loader {}

impl nakago_figment::Loader for Loader {
    fn load(&self, figment: Figment) -> Figment {
        // Split the HTTP variables
        figment
            .merge(Env::prefixed("HTTP").map(|key| key.as_str().replace("HTTP_", "HTTP.").into()))
    }
}

/// Add the default HTTP Config Loaders to the stack.
pub async fn add_default_loaders(i: &Inject) -> nakago::Result<()> {
    loaders::Add::default().loaders(i, default_loaders()).await
}

/// Add the default HTTP Config Loaders to the stack.
pub async fn add_default_loaders_with_tag(
    i: &Inject,
    tag: &'static Tag<Loaders>,
) -> nakago::Result<()> {
    loaders::Add::default()
        .with_tag(tag)
        .loaders(i, default_loaders())
        .await
}
