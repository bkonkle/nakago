//! # nakago-axum: An Axum HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// The top-level Applicaiton
pub mod app;

/// Application routing
pub mod router;

/// HTTP config
pub mod config;

/// Dependency injection Providers
pub mod providers;

/// Authentication
pub mod auth;

#[macro_use]
extern crate log;

pub use app::Application;
pub use providers::HttpConfigLoaders;
