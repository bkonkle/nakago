//! # The Async-GraphQL Example
#![forbid(unsafe_code)]

/// HTTP Entry Points
pub mod http;

/// Application config
pub mod config;

/// App Initialization
pub mod init;

/// `WebSocket` Events
pub mod events;

/// Application domains
pub mod domains;

/// Application utils
mod utils;

/// Error macros
#[macro_use]
extern crate anyhow;

pub use config::{Config, CONFIG};
