//! # The Async-GraphQL Example
#![forbid(unsafe_code)]

/// HTTP Handlers
pub mod handlers;

/// Axum Routes
pub mod routes;

/// Application config
pub mod config;

/// App Initialization
pub mod init;

/// Users
pub mod users;

/// Error macros
#[macro_use]
extern crate anyhow;
