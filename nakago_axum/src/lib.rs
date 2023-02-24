//! # nakago-axum: An Axum HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// The top-level Applicaiton
pub mod app;

/// HTTP config
pub mod config;

/// Dependency injection Providers
pub mod providers;

/// Authentication
pub mod auth;

#[macro_use]
extern crate log;

pub use app::HttpApplication;
pub use providers::init_config_loaders;
