use std::{any::Any, marker::PhantomData, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use figment::{
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::{inject, Inject};

use super::{Loader, CONFIG_LOADERS};

/// Config is the final initialized result
pub trait Config:
    Any + Clone + Default + Serialize + Send + Sync + for<'a> Deserialize<'a>
{
}

// /// A Config Provider
// ///
// /// **Provides:**
// ///   - `C: Config`
// ///
// /// **Depends on:**
// ///   - `Tag(ConfigLoaders)`
// #[derive(Default)]
// pub struct Provider<C: Config> {
//     custom_path: Option<PathBuf>,
//     _phantom: PhantomData<C>,
// }

// impl<C: Config> Provider<C> {
//     /// Create a new Config Initializer
//     pub fn new(custom_path: Option<PathBuf>) -> Self {
//         Self {
//             custom_path,
//             _phantom: PhantomData,
//         }
//     }

//     /// Create a new Config Initializer with a custom path
//     pub fn with_path(custom_path: PathBuf) -> Self {
//         Self {
//             custom_path: Some(custom_path),
//             _phantom: PhantomData,
//         }
//     }
// }

// #[async_trait]
// impl<C: Config> inject::Provider<C> for Provider<C> {
//     async fn provide(&self, i: &Inject) -> inject::Result<C> {
//         let loaders = i.get(&CONFIG_LOADERS).ok();

//         let config = load::<C>(loaders, self.custom_path.clone())
//             .map_err(|e| inject::Error::Provider(e.into()))?;

//         Ok(config)
//     }
// }

fn load<C: Config>(
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
