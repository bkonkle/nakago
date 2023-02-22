use figment::{
    providers::{Env, Format, Json, Serialized, Toml, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::{any::Any, marker::PhantomData, path::PathBuf};

/// A ConfigLoader uses hooks to augment the Config loaded for the application
///
/// TODO: Add more hooks! 🙂
pub trait ConfigLoader: Any + Send + Sync {
    /// Apply transformations to the environment variables loaded by Figment
    fn load_env(&self, env: Env) -> Env;
}

/// Config is the final loaded result
pub trait Config:
    Any + Clone + Default + Serialize + Send + Sync + for<'a> Deserialize<'a>
{
}

/// An extensible Config loader based on Figment
pub struct Loader<C: Config> {
    loaders: Vec<Box<dyn ConfigLoader>>,
    _phantom: PhantomData<C>,
}

impl<C: Config> Loader<C> {
    /// Create a new Config instance with the given loaders
    pub fn new(loaders: Vec<Box<dyn ConfigLoader>>) -> Self {
        Self {
            loaders,
            _phantom: Default::default(),
        }
    }

    /// Create a new Config by merging in various sources
    pub fn load(&self, custom_path: &Option<PathBuf>) -> figment::error::Result<C> {
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

        for loader in &self.loaders {
            env = loader.load_env(env);
        }

        println!(">- env (after) -> {:?}", env);

        config = config.merge(env);

        println!(">- config -> {:?}", config);

        // Serialize and freeze
        config.extract()
    }
}
