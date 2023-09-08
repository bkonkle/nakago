use axum::extract::FromRef;
use nakago::{Config, Tag};
use nakago_axum::{auth::config::AuthConfig, config::HttpConfig};
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(AppConfig)
pub const CONFIG: Tag<AppConfig> = Tag::new("AppConfig");

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct AppConfig {
    /// HTTP config
    pub http: HttpConfig,

    /// HTTP Auth Config
    pub auth: AuthConfig,
}

impl Config for AppConfig {}
