use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use tokio::sync::Mutex;

use super::loader::{Config, ConfigLoader, Loader};
use crate::inject;

/// A Tag for Config loaders
pub const CONFIG_LOADERS: inject::Tag<Mutex<Vec<Arc<dyn ConfigLoader>>>> =
    inject::Tag::new("ConfigLoaders");

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
impl<C: Config> inject::Hook for InitConfig<C> {
    async fn handle(&self, i: &mut inject::Inject) -> inject::Result<()> {
        if let Ok(loaders) = i.get(&CONFIG_LOADERS).await {
            let loader = Loader::<C>::new(loaders.lock().await.clone());

            let config = loader
                .load(self.custom_path.clone())
                .map_err(|e| inject::Error::Provider(Arc::new(e.into())))?;

            i.inject_type(config)?;
        }

        Ok(())
    }
}

/// Add the given Config Loaders to the stack. Injects `Tag(ConfigLoaders)` if it has not been
/// provided yet.
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
impl inject::Hook for AddConfigLoaders {
    async fn handle(&self, i: &mut inject::Inject) -> inject::Result<()> {
        if let Ok(existing) = i.get(&CONFIG_LOADERS).await {
            let mut existing = existing.lock().await;

            // Add the given ConfigLoaders to the stack
            for loader in self.loaders.iter() {
                existing.push(loader.clone());
            }
        } else {
            i.inject(&CONFIG_LOADERS, Mutex::new(self.loaders.clone()))?;
        }

        Ok(())
    }
}
