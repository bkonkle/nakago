use async_trait::async_trait;
use nakago::{
    config::{loader::ConfigLoader, providers::CONFIG_LOADERS},
    inject,
};

use crate::auth::config::AuthConfigLoader;

/// Initialize the ConfigLoaders needed for Axum integration
#[derive(Default)]
pub struct HttpConfigLoaders {}

#[async_trait]
impl inject::Initializer for HttpConfigLoaders {
    /// Add the HttpConfigLoader to the ConfigLoaders list
    async fn init(&self, i: &mut inject::Inject) -> inject::Result<()> {
        if let Ok(loaders) = i.get_tag_mut(&CONFIG_LOADERS) {
            // Add the AuthConfigLoader to the stack
            loaders.push(Box::new(AuthConfigLoader::default()));
        }

        if let Ok(loaders) = i.get_mut::<Vec<Box<dyn ConfigLoader>>>() {
            // Add the AuthConfigLoader to the stack
            loaders.push(Box::new(AuthConfigLoader::default()));
        }

        Ok(())
    }
}
