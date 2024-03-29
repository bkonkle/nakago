use nakago::{self, Tag};
use nakago_axum::{self, auth};
use nakago_derive::FromRef;
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(app::Config)
pub const CONFIG: Tag<Config> = Tag::new("app::Config");

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// HTTP Auth Config
    pub auth: auth::Config,
}

impl nakago::Config for Config {}
