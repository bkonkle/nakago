use async_trait::async_trait;
use nakago::{config::AddConfigLoaders, Dependency, Inject, InjectResult, Provider, Tag};
use oso::Oso;
use std::sync::Arc;

use crate::config::DatabaseConfigLoader;

/// The Oso Tag
pub const OSO: Tag<Oso> = Tag::new("Oso");

/// Provide an Oso authorization instance
///
/// **Provides:** `Oso`
#[derive(Default)]
pub struct ProvideOso {}

#[async_trait]
impl Provider for ProvideOso {
    async fn provide(self: Arc<Self>, _i: Inject) -> InjectResult<Arc<Dependency>> {
        Ok(Arc::new(Oso::new()))
    }
}

/// Add the Config Loaders that are custom to this app
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
pub fn add_app_config_loaders() -> AddConfigLoaders {
    AddConfigLoaders::new(vec![Arc::<DatabaseConfigLoader>::default()])
}
