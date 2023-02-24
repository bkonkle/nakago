use std::path::PathBuf;

use super::loader::{Config, ConfigLoader, Loader};
use crate::inject;

/// A Tag for Config loaders
pub const CONFIG_LOADERS: inject::Tag<Vec<Box<dyn ConfigLoader>>> =
    inject::Tag::new("ConfigLoaders");

/// A Config Initializer
///
/// **Provides:**
///   - `C: Config`
///
/// **Consumes:**
///   - `Tag(ConfigLoaders)`
pub async fn init<C: Config>(
    i: &mut inject::Inject,
    custom_path: Option<PathBuf>,
) -> inject::Result<()> {
    let loaders = i.consume(&CONFIG_LOADERS).unwrap_or_default();
    let loader = Loader::<C>::new(loaders);

    let config = loader
        .load(custom_path)
        .map_err(|e| inject::Error::Provider(e.into()))?;

    i.inject_type(config)?;

    Ok(())
}

/// Initialize the ConfigLoaders needed for Axum integration. Injects `Tag(ConfigLoaders)` if it
/// has not been provided yet.
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
pub async fn init_loaders(
    i: &mut inject::Inject,
    loaders: Vec<Box<dyn ConfigLoader>>,
) -> inject::Result<()> {
    if let Ok(existing) = i.get_mut(&CONFIG_LOADERS) {
        // Add the given ConfigLoaders to the stack
        for loader in loaders {
            existing.push(loader);
        }
    } else {
        i.inject(&CONFIG_LOADERS, loaders)?;
    }

    Ok(())
}
