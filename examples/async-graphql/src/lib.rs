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
mod utils;

/// Database utils
mod db;

/// GraphQL utils
mod graphql;

/// Error macros
#[macro_use]
extern crate anyhow;
