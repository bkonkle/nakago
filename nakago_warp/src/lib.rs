//! # nakago-warp: A Warp HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// The top-level Applicaiton
pub mod app;

/// HTTP config
pub mod config;

/// Authentication
pub mod auth;

/// Errors
pub mod errors;

/// Routes
pub mod routes;

/// Testing
pub mod test;

#[macro_use]
extern crate log;

pub use app::WarpApplication;
pub use config::Config;
pub use routes::Route;
