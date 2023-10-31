use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use async_trait::async_trait;

use crate::{inject, Hook, Inject, InjectError, Tag};

use super::loader::{Config, LoadAll, Loader};

/// A Tag for Config loaders
pub const LOADERS: Tag<Vec<Arc<dyn Loader>>> = Tag::new("config::Loaders");

/// Add the given Config Loaders to the stack.
///
/// **Provides or Modifies:**
///   - `Tag(config::Loaders)`
pub struct AddLoaders {
    loaders: Vec<Arc<dyn Loader>>,
}

impl AddLoaders {
    /// Create a new AddLoaders instance
    pub fn new(loaders: Vec<Arc<dyn Loader>>) -> Self {
        Self { loaders }
    }
}

#[async_trait]
impl Hook for AddLoaders {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let loaders = match i.consume(&LOADERS).await {
            Ok(loaders) => {
                let mut updated = loaders.clone();

                // Add the given ConfigLoaders to the stack
                for loader in self.loaders.iter() {
                    updated.push(loader.clone());
                }

                updated
            }
            Err(_) => self.loaders.clone(),
        };

        i.inject(&LOADERS, loaders).await?;

        Ok(())
    }
}

/// A Config Initializer
///
/// **Provides:**
///   - `Config`
///
/// **Consumes:**
///   - `Tag(config::Loaders)`
#[derive(Default)]
pub struct Init<C: Config> {
    custom_path: Option<PathBuf>,
    tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<C>,
}

impl<C: Config> Init<C> {
    /// Create a new Init instance
    pub fn new(custom_path: Option<PathBuf>, tag: Option<&'static Tag<C>>) -> Self {
        Self {
            custom_path,
            tag,
            _phantom: PhantomData,
        }
    }

    /// Use a custom path when loading the Config
    pub fn with_path(self, custom_path: PathBuf) -> Self {
        Self {
            custom_path: Some(custom_path),
            ..self
        }
    }

    /// Use a config Tag when injecting the loaded Config
    pub fn with_tag(self, tag: &'static Tag<C>) -> Self {
        Self {
            tag: Some(tag),
            ..self
        }
    }
}

#[async_trait]
impl<C: Config> Hook for Init<C> {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let loaders = i.get(&LOADERS).await.unwrap_or_default().to_vec();
        let loader = LoadAll::<C>::new(loaders);

        let config = loader
            .load(self.custom_path.clone())
            .map_err(|e| InjectError::Provider(Arc::new(e.into())))?;

        if let Some(tag) = self.tag {
            i.inject(tag, config).await?;
        } else {
            i.inject_type(config).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use anyhow::Result;
    use figment::providers::Env;

    use crate::config::loader::test::Config;

    use super::*;

    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct TestLoader {}

    impl Loader for TestLoader {
        fn load_env(&self, env: Env) -> Env {
            env
        }
    }

    #[tokio::test]
    async fn test_add_loaders_success() -> Result<()> {
        let i = Inject::default();

        let loader: Arc<dyn Loader> = Arc::new(TestLoader::default());

        let hook = AddLoaders::new(vec![loader]);

        i.handle(hook).await?;

        let results = i.get(&LOADERS).await?;
        assert_eq!(results.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_loaders_to_existing() -> Result<()> {
        let i = Inject::default();

        let loader: Arc<dyn Loader> = Arc::new(TestLoader::default());

        let existing: Vec<Arc<dyn Loader>> = vec![Arc::new(TestLoader::default())];

        i.inject(&LOADERS, existing).await?;

        let hook = AddLoaders::new(vec![loader]);

        i.handle(hook).await?;

        let results = i.get(&LOADERS).await?;
        assert_eq!(results.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_init_success() -> Result<()> {
        let i = Inject::default();

        let hook = Init::<Config>::new(None, None);

        i.handle(hook).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_init_custom_path() -> Result<()> {
        let i = Inject::default();

        let custom_path = PathBuf::from("config.toml");

        let hook = Init::<Config>::new(Some(custom_path), None);

        i.handle(hook).await?;

        Ok(())
    }
}
