use std::{any::Any, path::PathBuf, pin::Pin, sync::Arc};

use figment::{
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
    Figment,
};
use futures::Future;
use serde::{Deserialize, Serialize};

use crate::{inject::container::Dependency, Inject, InjectError, InjectResult, Tag};

/// A Tag for Config loaders
pub const CONFIG_LOADERS: Tag<Vec<Arc<dyn Loader>>> = Tag::new("ConfigLoaders");

/// Config is the final initialized result
pub trait Config:
    Any + Clone + Default + Serialize + Send + Sync + for<'a> Deserialize<'a>
{
}

/// A ConfigLoader uses hooks to augment the Config loaded for the application
///
/// TODO: Add more transformation hooks! 🙂
pub trait Loader: Any + Send + Sync {
    /// Apply transformations to the environment variables loaded by Figment
    fn load_env(&self, env: Env) -> Env;
}

pub fn provide_config<'a, C: Config>(
    custom_path: Option<PathBuf>,
) -> impl FnOnce(&'a Inject) -> Pin<Box<dyn Future<Output = InjectResult<Arc<Dependency>>> + 'a>> {
    |i| {
        Box::pin(async move {
            let loaders = i.get(&CONFIG_LOADERS).await?;

            let config = load::<C>(&loaders, custom_path)
                .map_err(|err| InjectError::Provider(Arc::new(err.into())))?;

            let dependency: Arc<Dependency> = Arc::new(config);

            Ok(dependency)
        })
    }
}

/// Load a Config with the given loaders, with an optional custom path
fn load<C: Config>(
    loaders: &Vec<Arc<dyn Loader>>,
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

    for loader in loaders {
        env = loader.load_env(env);
    }

    config = config.merge(env);

    // Serialize and freeze
    config.extract()
}
