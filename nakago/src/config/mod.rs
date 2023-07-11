/// Config Loaders
pub mod loader;

pub mod hooks;

pub use hooks::add_loaders;
pub use loader::{Config, Loader, CONFIG_LOADERS};
