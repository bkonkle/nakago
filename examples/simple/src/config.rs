use axum::extract::FromRef;
use nakago::Tag;
use nakago_axum::config::HttpConfig;
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(AppConfig)
pub const CONFIG: Tag<AppConfig> = Tag::new("AppConfig");

/// Server Config
#[derive(Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct AppConfig {
    /// HTTP config
    pub http: HttpConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            http: HttpConfig {
                port: 0,
                address: "0.0.0.0".to_string(),
            },
        }
    }
}

impl nakago::Config for AppConfig {}
