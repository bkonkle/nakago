use nakago::config::AddConfigLoaders;
use std::sync::Arc;

use crate::auth::config::AuthConfigLoader;

/// Add the Config Loaders that are custom to this app
pub fn add_http_config_loaders() -> AddConfigLoaders {
    AddConfigLoaders::new(vec![Arc::<AuthConfigLoader>::default()])
}
