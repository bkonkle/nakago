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
    _phantom: PhantomData<C>,
}

impl<C: Config> InitConfig<C> {
    /// Create a new InitConfig
    pub fn new(custom_path: Option<PathBuf>) -> Self {
        Self {
            custom_path,
            _phantom: PhantomData,
        }
    }

    /// Create a new InitConfig with a custom path
    pub fn with_path(custom_path: PathBuf) -> Self {
        Self {
            custom_path: Some(custom_path),
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<C: Config> Hook for InitConfig<C> {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        println!(">------ InitConfig ------<");

        if let Ok(loaders) = i.get(&CONFIG_LOADERS).await {
            println!(">------ Ok(loaders) ------<");
            let loader = Loader::<C>::new(loaders.to_vec());

            let config = loader
                .load(self.custom_path.clone())
                .map_err(|e| InjectError::Provider(Arc::new(e.into())))?;

            i.inject_type(config).await?;
        }

        Ok(())
    }
}
