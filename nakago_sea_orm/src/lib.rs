//! # nakago-sea-orm: A SeaORM integration for Nakago
#![forbid(unsafe_code)]

/// Database Connections
pub mod connection;

/// Database Config
pub mod config;

pub use config::{Config, DatabasePool};
pub use connection::CONNECTION;

// Re-exports
pub use sea_orm::DatabaseConnection;
