use nakago_derive::FromRef;
use nakago_warp::{self, auth};
use serde::Serialize;
use serde_derive::Deserialize;

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_warp::Config,

    /// HTTP Auth Config
    pub auth: auth::Config,
}

impl nakago_figment::Config for Config {}
