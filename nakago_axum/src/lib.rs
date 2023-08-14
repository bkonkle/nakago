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

#[macro_use]
extern crate log;

pub use app::AxumApplication;
pub use config::add_http_config_loaders;
pub use routes::{InitRoute, Route};
