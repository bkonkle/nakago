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
pub use auth::ConfigLoader as AuthConfigLoader;
pub use routes::InitRouter;
