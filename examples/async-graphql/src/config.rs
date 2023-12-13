use nakago::Tag;
use nakago_axum::{self, auth};
use nakago_derive::FromRef;
use nakago_sea_orm::{self, config::DatabasePool};
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(Config)
pub const CONFIG: Tag<Config> = Tag::new("app::Config");

/// Server Config
#[derive(Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// HTTP Auth Config
    pub auth: auth::Config,

    /// Database config
    pub database: nakago_sea_orm::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http: nakago_axum::Config {
                port: 0,
                address: "0.0.0.0".to_string(),
            },
            auth: auth::Config {
                url: "https://async-graphql.us.auth0.com".to_string(),
                audience: "localhost".to_string(),
                client: auth::config::Client::default(),
            },
            database: nakago_sea_orm::Config {
                url: "postgresql://async-graphql:async-graphql@localhost:5432/async-graphql"
                    .to_string(),
                debug: false,
                pool: DatabasePool::default(),
            },
        }
    }
}

impl nakago::Config for Config {}
