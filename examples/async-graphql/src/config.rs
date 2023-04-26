use axum::extract::FromRef;
use figment::providers::Env;
use nakago::ConfigLoader;
use nakago_axum::{
    auth::config::{AuthClientConfig, AuthConfig},
    config::HttpConfig,
};
use serde::Serialize;
use serde_derive::Deserialize;

/// Server Config
#[derive(Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct AppConfig {
    /// HTTP config
    pub http: HttpConfig,

    /// HTTP Auth Config
    pub auth: AuthConfig,

    /// Database config
    pub database: Database,
}

/// Database pool config
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DbPool {
    /// Database pool min
    pub min: Option<i16>,

    /// Database pool max
    pub max: Option<i16>,
}

/// Database config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    /// Database hostname/IP
    pub hostname: String,

    /// Database username
    pub username: String,

    /// Database password
    pub password: String,

    /// Database name
    pub name: String,

    /// Database port
    pub port: u16,

    /// Full database url
    pub url: String,

    /// Database debug logging
    pub debug: bool,

    /// Database pool config
    pub pool: DbPool,
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
            database: Database {
                hostname: "localhost".to_string(),
                username: "async-graphql".to_string(),
                password: "async-graphql".to_string(),
                name: "async-graphql".to_string(),
                port: 1701,
                url: "postgresql://async-graphql:async-graphql@localhost:1701/async-graphql"
                    .to_string(),
                debug: false,
                pool: DbPool::default(),
            },
        }
    }
}

impl nakago::Config for AppConfig {}

/// The Database Config Loader
#[derive(Default)]
pub struct DatabaseConfigLoader {}

impl ConfigLoader for DatabaseConfigLoader {
    fn load_env(&self, env: Env) -> Env {
        // Split the Database variables
        env.map(|key| {
            key.as_str()
                .replace("DATABASE_POOL_", "DATABASE.POOL.")
                .into()
        })
        .map(|key| key.as_str().replace("DATABASE_", "DATABASE.").into())
    }
}
