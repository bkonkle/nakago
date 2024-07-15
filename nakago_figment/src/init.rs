use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use nakago::{Error, Inject, Result, Tag};

use crate::{loader::LoadAll, Config, Loaders};

/// Add Loaders
/// ===========

/// Add the given Config Loaders to the stack currently in the injection container.
pub async fn add_loaders(i: &Inject, loaders: Loaders) -> Result<()> {
    add_loaders_impl(i, loaders, None).await
}

/// Add the given Config Loaders to the stack currently in the injection container for the given
/// tag.
pub async fn add_loaders_with_tag(
    i: &Inject,
    loaders: Loaders,
    tag: &'static Tag<Loaders>,
) -> Result<()> {
    add_loaders_impl(i, loaders, Some(tag)).await
}

async fn add_loaders_impl(
    i: &Inject,
    loaders: Loaders,
    tag: Option<&'static Tag<Loaders>>,
) -> Result<()> {
    let current_result = match tag {
        Some(tag) => i.consume_tag(tag).await,
        None => i.consume::<Loaders>().await,
    };

    let current = match current_result {
        Ok(current) => {
            let mut updated = current.clone();

            // Add the given ConfigLoaders to the stack
            for loader in loaders.iter() {
                updated.push(loader.clone());
            }

            updated
        }
        Err(_) => loaders.clone(),
    };

    i.inject::<Loaders>(current).await?;

    Ok(())
}

/// Init
/// ====

#[derive(Default)]
pub struct Init<C: Config> {
    custom_path: Option<PathBuf>,
    loaders_tag: Option<&'static Tag<Loaders>>,
    config_tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<C>,
}

impl<C: Config> Init<C> {
    /// Create a new Init instance
    pub fn new(
        custom_path: Option<PathBuf>,
        loaders_tag: Option<&'static Tag<Loaders>>,
        config_tag: Option<&'static Tag<C>>,
    ) -> Self {
        Self {
            custom_path,
            config_tag,
            loaders_tag,
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

    /// Use a Config Tag when injecting the loaded Config
    pub fn with_config_tag(self, tag: &'static Tag<C>) -> Self {
        Self {
            config_tag: Some(tag),
            ..self
        }
    }

    /// Use a Loaders Tag when retrieving the current Loaders
    pub fn with_loaders_tag(self, tag: &'static Tag<Loaders>) -> Self {
        Self {
            loaders_tag: Some(tag),
            ..self
        }
    }

    /// Initialize the Config
    pub async fn init(&self, i: &Inject) -> Result<()> {
        let loaders_result = match self.loaders_tag {
            Some(tag) => i.get_tag(tag).await,
            None => i.get::<Loaders>().await,
        };

        let loaders = loaders_result.unwrap_or_default().to_vec();
        let loader = LoadAll::<C>::new(loaders);

        let config = loader
            .load(self.custom_path.clone())
            .extract()
            .map_err(|e| Error::Any(Arc::new(e.into())))?;

        if let Some(tag) = self.config_tag {
            i.inject_tag(tag, config).await?;
        } else {
            i.inject::<C>(config).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use figment::Figment;

    use crate::{loader::test::Config, Loader};

    use super::*;

    /// A Tag for Config loaders
    pub const LOADERS: Tag<Vec<Arc<dyn Loader>>> = Tag::new("config::Loaders");

    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct TestLoader {}

    impl Loader for TestLoader {
        fn load(&self, figment: Figment) -> Figment {
            figment
        }
    }

    #[tokio::test]
    async fn test_add_loaders_success_with_type() -> Result<()> {
        let i = Inject::default();

        let loader: Arc<dyn Loader> = Arc::new(TestLoader::default());

        add_loaders(&i, vec![loader]).await?;

        let results = i.get::<Loaders>().await?;
        assert_eq!(results.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_loaders_success_with_tag() -> Result<()> {
        let i = Inject::default();

        let loader: Arc<dyn Loader> = Arc::new(TestLoader::default());

        add_loaders_with_tag(&i, vec![loader], &LOADERS).await?;

        let results = i.get_tag(&LOADERS).await?;
        assert_eq!(results.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_loaders_to_existing_with_type() -> Result<()> {
        let i = Inject::default();

        let loader: Arc<dyn Loader> = Arc::new(TestLoader::default());

        let existing: Vec<Arc<dyn Loader>> = vec![Arc::new(TestLoader::default())];

        i.inject::<Loaders>(existing).await?;

        add_loaders(&i, vec![loader]).await?;

        let results = i.get::<Loaders>().await?;
        assert_eq!(results.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_loaders_to_existing_with_tag() -> Result<()> {
        let i = Inject::default();

        let loader: Arc<dyn Loader> = Arc::new(TestLoader::default());

        let existing: Vec<Arc<dyn Loader>> = vec![Arc::new(TestLoader::default())];

        i.inject_tag(&LOADERS, existing).await?;

        add_loaders_with_tag(&i, vec![loader], &LOADERS).await?;

        let results = i.get_tag(&LOADERS).await?;
        assert_eq!(results.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_init_success() -> Result<()> {
        let i = Inject::default();

        let hook = Init::<Config>::default();
        assert!(hook.custom_path.is_none());
        assert!(hook.loaders_tag.is_none());
        assert!(hook.config_tag.is_none());

        hook.init(&i).await?;

        let hook = Init::<Config>::default().with_path("TEST_PATH".into());
        assert!(hook.custom_path.is_some());

        let hook = Init::<Config>::default().with_loaders_tag(&LOADERS);
        assert!(hook.config_tag.is_none());
        assert!(hook.loaders_tag.is_some());

        hook.init(&i).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_init_custom_path() -> Result<()> {
        let i = Inject::default();

        let custom_path = PathBuf::from("config.toml");

        let hook = Init::<Config>::new(Some(custom_path), None, None);

        hook.init(&i).await?;

        Ok(())
    }
}
