use std::{any::Any, path::PathBuf, sync::Arc};

use figment::{
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

use super::Loader;

/// Config is the final initialized result
pub trait Config:
    Any + Clone + Default + Serialize + Send + Sync + for<'a> Deserialize<'a>
{
}

/// Load a Config with the given loaders, with an optional custom path
pub fn load<C: Config>(
    loaders: Option<&Vec<Arc<dyn Loader>>>,
    custom_path: Option<PathBuf>,
) -> figment::error::Result<C> {
    let mut config = Figment::new()
        // Load defaults
        .merge(Serialized::defaults(C::default()))
        // Load local overrides
        .merge(Toml::file("config.toml"))
        .merge(Yaml::file("config.yml"))
        .merge(Yaml::file("config.yaml"))
        .merge(Json::file("config.json"));

    // Load the custom config file if provided
    if let Some(path) = custom_path {
        if let Some(path_str) = path.to_str() {
            if path_str.ends_with(".toml") {
                config = config.merge(Toml::file(path_str));
            } else if path_str.ends_with(".yml") || path_str.ends_with(".yaml") {
                config = config.merge(Yaml::file(path_str));
            } else if path_str.ends_with(".json") {
                config = config.merge(Json::file(path_str));
            }
        }
    }

    // Environment Variables
    // ---------------------

    let mut env = Env::raw();

    if let Some(loaders) = loaders {
        for loader in loaders {
            env = loader.load_env(env);
        }
    }

    config = config.merge(env);

    // Serialize and freeze
    config.extract()
}
