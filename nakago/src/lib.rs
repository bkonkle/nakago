//! # Nakago: The lightweight Rust framework for sharp services 😎
#![forbid(unsafe_code)]

/// Dependency Injection
pub mod inject;

/// Configuration utilities based on Figment
pub mod config;

/// Application initialization
pub mod app;

/// Lifecycle hooks
pub mod lifecycle;

pub use app::Application;
pub use config::loader::{Config, ConfigLoader};
pub use inject::{
    to_provider_error, Dependency, Error as InjectError, Hook, Inject, Provider,
    Result as InjectResult, Tag,
};
pub use lifecycle::EventType;
