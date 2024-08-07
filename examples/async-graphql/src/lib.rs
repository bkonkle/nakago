//! # The Async-GraphQL Example
#![forbid(unsafe_code)]

/// HTTP Entry Points
pub mod http;

/// Application config
pub mod config;

/// App Initialization
pub mod init;

/// `WebSocket` Messages
pub mod messages;

/// Application domains
pub mod domains;

/// Authorization
pub mod authz;

/// Error macros
#[macro_use]
extern crate anyhow;

pub use config::Config;
