use async_trait::async_trait;
use std::{marker::PhantomData, path::PathBuf};

use super::loader::{Config, ConfigData, ConfigLoader};
use crate::inject;

pub const CONFIG_LOADERS: inject::Tag<Vec<Box<dyn ConfigLoader>>> =
    inject::Tag::new("ConfigLoaders");

/// The dependency injection Provider for the Config
///
/// **Provides:** `C: ConfigData`
///
/// **Consumes:**
///   - `Tag(ConfigLoaders)`
#[derive(Default)]
pub struct ConfigProvider<C: ConfigData> {
    custom_path: Option<PathBuf>,
    _phantom: PhantomData<C>,
}

#[async_trait]
impl<C: ConfigData> inject::Provider<C> for ConfigProvider<C> {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<C> {
        let loaders = i.consume_tag(&CONFIG_LOADERS)?;
        let config = Config::new(loaders);

        Ok(config.load(self.custom_path)?)
    }
}
