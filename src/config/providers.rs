use std::{marker::PhantomData, path::PathBuf};

use async_trait::async_trait;

use super::loader::{Config, ConfigLoader, Loader};
use crate::inject;

/// A Tag for Config loaders
pub const CONFIG_LOADERS: inject::Tag<Vec<Box<dyn ConfigLoader>>> =
    inject::Tag::new("ConfigLoaders");

/// A Config Initializer
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
impl<C: Config> inject::Initializer for ConfigInitializer<C> {
    async fn init(&self, i: &mut inject::Inject) -> inject::Result<()> {
        let mut loaders = i.consume_tag(&CONFIG_LOADERS).unwrap_or_default();

        loaders.extend(
            i.consume::<Vec<Box<dyn ConfigLoader>>>()
                .unwrap_or_default(),
        );

        let config = Loader::<C>::new(loaders);

        let data = config
            .load(&self.custom_path)
            .map_err(|e| inject::Error::Provider(e.into()))?;

        i.inject(data)?;

        Ok(())
    }
}
