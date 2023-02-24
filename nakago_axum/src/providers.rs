use async_trait::async_trait;
use nakago::{config::providers::CONFIG_LOADERS, inject};

use crate::auth::config::AuthConfigLoader;

/// Initialize the ConfigLoaders needed for Axum integration. Injects `Tag(ConfigLoaders)` if it
/// has not been provided yet.
#[derive(Default)]
pub struct HttpConfigLoaders {}

#[async_trait]
impl inject::Initializer for HttpConfigLoaders {
    /// Add the HttpConfigLoader to the ConfigLoaders list
    async fn init(&self, i: &mut inject::Inject) -> inject::Result<()> {
        if let Ok(loaders) = i.get_mut(&CONFIG_LOADERS) {
            // Add the AuthConfigLoader to the stack
            loaders.push(Box::<AuthConfigLoader>::default());
        } else {
            i.inject(&CONFIG_LOADERS, vec![Box::<AuthConfigLoader>::default()])?;
        }

        Ok(())
    }
}
