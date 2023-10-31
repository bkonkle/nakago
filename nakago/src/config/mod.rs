/// The extensible Config Loader
pub mod loader;

/// Dependency injection Hooks
pub mod hooks;

pub use hooks::{AddLoaders, Init, LOADERS};
pub use loader::{Config, Loader};
