//! # nakago-axum: An Axum HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// The top-level Applicaiton
pub mod app;

/// HTTP config
pub mod config;

/// Authentication
pub mod auth;

/// Routes
pub mod routes;

/// Axum State
pub mod state;

/// Testing
pub mod test;

/// Errors
pub mod errors;

/// Utils
pub mod utils;

#[macro_use]
extern crate log;

pub use app::AxumApplication;
pub use config::Config;
pub use errors::Error;
pub use routes::Route;
pub use state::{Inject, State};
