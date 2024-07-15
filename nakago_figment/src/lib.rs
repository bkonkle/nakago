//! # nakago-ws: A Warp HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// The extensible Config Loader
pub mod loader;

/// Config init routines
pub mod init;

pub use loader::{Config, Loader, Loaders};
