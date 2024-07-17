//! # nakago-axum: An Axum HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// HTTP config
pub mod config;

/// Authentication
pub mod auth;

/// Axum State
pub mod state;

/// Service Initialization Helpers
pub mod init;

/// Testing
pub mod test;

/// Errors
pub mod errors;

/// Utils
pub mod utils;

#[macro_use]
extern crate log;

pub use config::Config;
pub use errors::Error;
pub use state::{Inject, State};
