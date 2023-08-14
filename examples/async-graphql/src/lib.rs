//! # The Async-GraphQL Example
#![forbid(unsafe_code)]

/// HTTP Handlers
pub mod handlers;

/// `WebSocket` Events
pub mod events;

/// Axum Routes
pub mod routes;

/// Application domains
pub mod domains;

/// Application config
pub mod config;

/// Application utils
pub mod utils;

/// GraphQL utils
pub mod graphql;

/// App Initialization
pub mod init;

/// Error macros
#[macro_use]
extern crate anyhow;
