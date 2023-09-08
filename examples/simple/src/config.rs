use axum::extract::FromRef;
use nakago::{Config, Tag};
use nakago_axum::config::HttpConfig;
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(AppConfig)
pub const CONFIG: Tag<AppConfig> = Tag::new("AppConfig");

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct AppConfig {
    /// HTTP config
    pub http: HttpConfig,
}

impl Config for AppConfig {}
