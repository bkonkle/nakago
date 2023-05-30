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

/// RUST_LOG initialization
pub mod log;

/// Panic handler
pub mod panic;

pub use app::Application;
pub use config::{
    AddLoaders as AddConfigLoaders, Config, Loader as ConfigLoader, Provider as ConfigProvider,
};
pub use inject::{
    provide, to_provider_error, Error as InjectError, Hook, Hooks, Inject, Provide,
    Result as InjectResult, Tag,
};
pub use lifecycle::EventType;
