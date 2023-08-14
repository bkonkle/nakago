//! # The Async-GraphQL Example
#![forbid(unsafe_code)]

/// HTTP Handlers
pub mod handlers;

/// `WebSocket` Events
pub mod events;

/// Dependency Injection providers
pub mod providers;

/// Axum Routes
pub mod routes;

/// Application domains
pub mod domains;

/// Application config
pub mod config;

/// Application utils
pub mod utils;

/// Database utils
pub mod db;

/// GraphQL utils
pub mod graphql;

/// Error macros
#[macro_use]
extern crate anyhow;
