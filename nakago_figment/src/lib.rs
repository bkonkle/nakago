//! # nakago-ws: A Warp HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// The extensible Config Loader
pub mod loader;

/// Config loader init helpers
pub mod loaders;

/// The FromRef Utility
pub mod from_ref;

pub use from_ref::FromRef;
pub use loader::{Config, Loader};
pub use loaders::Loaders;
