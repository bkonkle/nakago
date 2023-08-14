/// The extensible Config Loader
pub mod loader;

/// Dependency injection Hooks
pub mod hooks;

pub use hooks::{AddConfigLoaders, InitConfig, CONFIG_LOADERS};
pub use loader::{Config, ConfigLoader};
