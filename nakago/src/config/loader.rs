use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use figment::providers::Env;

use crate::{Hook, Inject, InjectResult, Tag};

/// A Tag for Config loaders
pub const CONFIG_LOADERS: Tag<Vec<Arc<dyn Loader>>> = Tag::new("ConfigLoaders");

/// A ConfigLoader uses hooks to augment the Config loaded for the application
///
/// TODO: Add more transformation hooks! 🙂
pub trait Loader: Any + Send + Sync {
    /// Apply transformations to the environment variables loaded by Figment
    fn load_env(&self, env: Env) -> Env;
}

/// Add the given Config Loaders to the stack. Injects `Tag(ConfigLoaders)` if it has not been
/// provided yet.
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
pub struct AddLoaders {
    loaders: Vec<Arc<dyn Loader>>,
}

impl AddLoaders {
    /// Create a new AddLoaders instance
    pub fn new(loaders: Vec<Arc<dyn Loader>>) -> Self {
        Self { loaders }
    }
}

#[async_trait]
impl Hook for AddLoaders {
    async fn handle(&self, i: &mut Inject) -> InjectResult<()> {
        if let Ok(existing) = i.get_mut(&CONFIG_LOADERS) {
            // Add the given ConfigLoaders to the stack
            for loader in self.loaders.clone() {
                existing.push(loader);
            }
        } else {
            i.inject(&CONFIG_LOADERS, self.loaders.clone())?;
        }

        Ok(())
    }
}
