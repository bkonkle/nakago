use nakago::config::AddConfigLoaders;
use std::sync::Arc;

use crate::config::DatabaseConfigLoader;

/// Add the Config Loaders that are custom to this app
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
pub fn init_config_loaders() -> AddConfigLoaders {
    AddConfigLoaders::new(vec![Arc::<DatabaseConfigLoader>::default()])
}
