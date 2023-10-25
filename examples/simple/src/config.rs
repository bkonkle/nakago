use axum::extract::FromRef;
use nakago::{self, Tag};
use nakago_axum::{self, auth};
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(AppConfig)
pub const CONFIG: Tag<Config> = Tag::new("AppConfig");

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// HTTP Auth Config
    pub auth: auth::Config,
}

impl nakago::Config for Config {}
