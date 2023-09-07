use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use async_trait::async_trait;

use super::loader::{Config, ConfigLoader, Loader};
use crate::{Hook, Inject, InjectError, InjectResult, Tag};

/// A Tag for Config loaders
pub const CONFIG_LOADERS: Tag<Vec<Arc<dyn ConfigLoader>>> = Tag::new("ConfigLoaders");

/// Add the given Config Loaders to the stack.
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
pub struct AddConfigLoaders {
    loaders: Vec<Arc<dyn ConfigLoader>>,
}

impl AddConfigLoaders {
    /// Create a new AddConfigLoaders instance
    pub fn new(loaders: Vec<Arc<dyn ConfigLoader>>) -> Self {
        Self { loaders }
    }
}

#[async_trait]
impl Hook for AddConfigLoaders {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let loaders = match i.consume(&CONFIG_LOADERS).await {
            Ok(loaders) => {
                let mut updated = loaders.clone();

                // Add the given ConfigLoaders to the stack
                for loader in self.loaders.iter() {
                    updated.push(loader.clone());
                }

                updated
            }
            Err(_) => self.loaders.clone(),
        };

        let _ = i.override_tag(&CONFIG_LOADERS, loaders).await?;

        Ok(())
    }
}

/// A Config Initializer
///
/// **Provides:**
///   - `C: Config`
///
/// **Consumes:**
///   - `Tag(ConfigLoaders)`
#[derive(Default)]
pub struct InitConfig<C: Config> {
    custom_path: Option<PathBuf>,
    tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<C>,
}

impl<C: Config> InitConfig<C> {
    /// Create a new InitConfig instance
    pub fn new(custom_path: Option<PathBuf>, tag: Option<&'static Tag<C>>) -> Self {
        Self {
            custom_path,
            tag,
            _phantom: PhantomData,
        }
    }

    /// Use a custom path when loading the Config
    pub fn with_path(self, custom_path: PathBuf) -> Self {
        Self {
            custom_path: Some(custom_path),
            ..self
        }
    }

    /// Use a config Tag when injecting the loaded Config
    pub fn with_tag(self, tag: &'static Tag<C>) -> Self {
        Self {
            tag: Some(tag),
            ..self
        }
    }
}

#[async_trait]
impl<C: Config> Hook for InitConfig<C> {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        if let Ok(loaders) = i.get(&CONFIG_LOADERS).await {
            let loader = Loader::<C>::new(loaders.to_vec());

            let config = loader
                .load(self.custom_path.clone())
                .map_err(|e| InjectError::Provider(Arc::new(e.into())))?;

            if let Some(tag) = self.tag {
                i.inject(tag, config).await?;
            } else {
                i.inject_type(config).await?;
            }
        }

        Ok(())
    }
}
