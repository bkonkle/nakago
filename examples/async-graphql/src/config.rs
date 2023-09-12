use axum::extract::FromRef;
use nakago::Tag;
use nakago_axum::{
    auth::config::{AuthClientConfig, AuthConfig},
    config::HttpConfig,
};
use nakago_sea_orm::config::{DatabaseConfig, DatabasePool};
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(AppConfig)
pub const CONFIG: Tag<AppConfig> = Tag::new("AppConfig");

/// Server Config
#[derive(Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct AppConfig {
    /// HTTP config
    pub http: HttpConfig,

    /// HTTP Auth Config
    pub auth: AuthConfig,

    /// Database config
    pub database: DatabaseConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            http: HttpConfig {
                port: 0,
                address: "0.0.0.0".to_string(),
            },
            auth: AuthConfig {
                url: "https://async-graphql.us.auth0.com".to_string(),
                audience: "localhost".to_string(),
                client: AuthClientConfig::default(),
            },
            database: DatabaseConfig {
                url: "postgresql://async-graphql:async-graphql@localhost:5432/async-graphql"
                    .to_string(),
                debug: false,
                pool: DatabasePool::default(),
            },
        }
    }
}

impl nakago::Config for AppConfig {}
