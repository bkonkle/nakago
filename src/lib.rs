//! # Nakago: The lightweight Rust framework for sharp services 😎
#![forbid(unsafe_code)]

/// Dependency Injection
pub mod inject;

/// Configuration utilities based on Figment
pub mod config;

pub use inject::{provide, Error as InjectError, Inject, Provider, Result as InjectResult, Tag};
