//! # Nakago: The lightweight Rust framework for sharp services 😎
#![forbid(unsafe_code)]

/// Dependency Injection
pub mod inject;

/// Configuration utilities based on Figment
pub mod config;

/// Application initialization
pub mod system;

pub use config::loader::{Config, ConfigLoader};
pub use inject::{
    provide, to_provider_error, Error as InjectError, Inject, Provider, Result as InjectResult, Tag,
};
pub use system::System;
