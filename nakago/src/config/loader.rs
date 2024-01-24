use std::{any::Any, fmt::Debug, marker::PhantomData, path::PathBuf, sync::Arc};

use figment::{
    providers::{Format, Json, Serialized, Toml, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

/// A Loader uses hooks to augment the Config loaded for the application
pub trait Loader: Any + Send + Sync {
    /// Apply transformations to the Figment provider
    fn load(&self, figment: Figment) -> Figment;
}

/// Config is the final loaded result
pub trait Config:
    Any + Clone + Debug + Default + Serialize + Send + Sync + for<'a> Deserialize<'a>
{
}

/// An extensible Config loader based on Figment
pub struct LoadAll<C: Config> {
    loaders: Vec<Arc<dyn Loader>>,
    _phantom: PhantomData<C>,
}

impl<C: Config> LoadAll<C> {
    /// Create a new Config instance with the given loaders
    pub fn new(loaders: Vec<Arc<dyn Loader>>) -> Self {
        Self {
            loaders,
            _phantom: Default::default(),
        }
    }

    /// Create a new Config by merging in various sources
    pub fn load(&self, custom_path: Option<PathBuf>) -> Figment {
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

        // Apply individual loaders to transform the Figment provider
        for loader in &self.loaders {
            config = loader.load(config);
        }

        config
    }
}

#[cfg(test)]
pub(crate) mod test {
    use anyhow::Result;

    use crate::Tag;

    use super::*;

    #[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
    pub struct Config {}

    impl crate::Config for Config {}

    /// Tag(app::Config)
    pub const CONFIG: Tag<Config> = Tag::new("app::Config");

    #[tokio::test]
    async fn test_load_all_success() -> Result<()> {
        let loader = LoadAll::<Config>::new(vec![]);

        let _config: Config = loader.load(None).extract()?;

        Ok(())
    }

    #[tokio::test]
    async fn test_load_all_custom_path() -> Result<()> {
        let loader = LoadAll::<Config>::new(vec![]);

        let _figment: Figment = loader.load(Some("config.toml".into()));
        let _figment: Figment = loader.load(Some("config.yml".into()));
        let _figment: Figment = loader.load(Some("config.yaml".into()));
        let _figment: Figment = loader.load(Some("config.json".into()));

        Ok(())
    }
}
