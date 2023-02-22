use std::{fmt::Debug, marker::PhantomData, path::PathBuf};

use async_trait::async_trait;

use super::loader::{Config, ConfigLoader, Loader};
use crate::inject;

/// A Tag for Config loaders
pub const CONFIG_LOADERS: inject::Tag<Vec<Box<dyn ConfigLoader>>> =
    inject::Tag::new("ConfigLoaders");

/// A Config Initializer
///
/// **Consumes:**
///   - `Tag(ConfigLoaders)`
#[derive(Default)]
pub struct ConfigInitializer<C: Config> {
    custom_path: Option<PathBuf>,
    _phantom: PhantomData<C>,
}

impl<C: Config> ConfigInitializer<C> {
    /// Create a new Config Initializer with a custom path
    pub fn with_custom_path(custom_path: PathBuf) -> Self {
        Self {
            custom_path: Some(custom_path),
            _phantom: Default::default(),
        }
    }
}

#[async_trait]
impl<C: Config + Debug> inject::Initializer for ConfigInitializer<C> {
    async fn init(&self, i: &mut inject::Inject) -> inject::Result<()> {
        let loaders = i.consume(&CONFIG_LOADERS).unwrap_or_default();
        println!(">- loaders -> {:?}", loaders.len());
        let loader = Loader::<C>::new(loaders);

        let config = loader
            .load(&self.custom_path)
            .map_err(|e| inject::Error::Provider(e.into()))?;

        println!(">- config -> {:?}", config);

        i.inject_type(config)?;

        Ok(())
    }
}
