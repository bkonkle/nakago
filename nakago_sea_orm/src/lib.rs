//! # nakago-sea-orm: A SeaORM integration for Nakago
#![forbid(unsafe_code)]

/// Database Connections
pub mod connection;

/// Database Config
pub mod config;

pub use config::{init_config_loaders, DatabaseConfig, DatabaseConfigLoader, DatabasePool};
pub use connection::{ProvideConnection, DATABASE_CONNECTION};

// Re-exports
pub use sea_orm::DatabaseConnection;
