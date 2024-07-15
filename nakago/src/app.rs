use std::{
    io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    panic::{self, PanicInfo},
    path::PathBuf,
    sync::Arc,
};

use backtrace::Backtrace;
use crossterm::{execute, style::Print};
use tracing_subscriber::prelude::*;

use crate::{
    config, hooks,
    inject::{self, from_hook_error, Hook},
    lifecycle::Events,
    Config, EventType, Inject, Tag,
};

/// The top-level Application struct
#[derive(Default)]
pub struct Application<C: Config> {
    events: Events,
    i: Inject,
    config_tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<C>,
}

impl<C: Config> Application<C> {
    /// Create a new Application instance
    pub fn new(config_tag: Option<&'static Tag<C>>) -> Self {
        Self {
            config_tag,
            ..Self::default()
        }
    }

    /// Add a config Tag to this Application instance
    pub fn with_config_tag(self, tag: &'static Tag<C>) -> Self {
        Self {
            config_tag: Some(tag),
            ..self
        }
    }
}

impl<C> Deref for Application<C>
where
    C: Config,
{
    type Target = Inject;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

impl<C> DerefMut for Application<C>
where
    C: Config,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.i
    }
}

impl<C> Application<C>
where
    C: Config,
{
    /// Set a new lifecycle hook that will fire on the given EventType
    pub fn on(&mut self, event: &EventType, hook: impl Hook) {
        self.events.on(event, hook);
    }

    /// Trigger the given lifecycle event
    pub async fn trigger(&self, event: &EventType) -> hooks::Result<()> {
        self.events.trigger(event, self.i.clone()).await
    }

    /// Load the App's dependencies and configuration. Triggers the Load lifecycle event.
    pub async fn load(&self, config_path: Option<PathBuf>) -> hooks::Result<()> {
        // Trigger the Load lifecycle event
        self.trigger(&EventType::Load)
            .await
            .map_err(from_hook_error)?;

        // Load the Config using the given path
        self.i
            .handle(config::Init::new(config_path, self.config_tag))
            .await?;

        Ok(())
    }

    /// Initialize the App and provide the top-level Config. Triggers the Init lifecycle event.
    pub async fn init(&self) -> hooks::Result<()> {
        // Trigger the Init lifecycle event
        self.trigger(&EventType::Init).await?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        // Process setup
        panic::set_hook(Box::new(handle_panic));

        Ok(())
    }

    /// Trigger the Startup lifecycle event.
    pub async fn start(&self) -> hooks::Result<()> {
        self.trigger(&EventType::Startup).await
    }

    /// Trigger the Shutdown lifecycle event.
    pub async fn stop(&self) -> hooks::Result<()> {
        self.trigger(&EventType::Shutdown).await
    }

    /// Get the top-level Config by tag or type
    pub async fn get_config(&self) -> inject::Result<Arc<C>> {
        let config = if let Some(tag) = self.config_tag {
            self.i.get_tag(tag).await?
        } else {
            self.i.get::<C>().await?
        };

        Ok(config)
    }
}

fn handle_panic(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        execute!(
            io::stdout(),
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            ))
        )
        .unwrap();
    }
}

#[cfg(test)]
pub mod test {
    use anyhow::Result;

    use crate::config::{
        hooks::test::TestLoader,
        loader::test::{Config, CONFIG},
        AddLoaders, Loader,
    };

    use super::*;

    #[tokio::test]
    async fn test_app_deref_success() -> Result<()> {
        let mut app = Application::<Config>::default();

        let keys = app.get_available_keys().await;
        assert_eq!(keys.len(), 0);

        let mut _m = &mut *app;

        Ok(())
    }

    #[tokio::test]
    async fn test_app_load_success() -> Result<()> {
        let mut app = Application::<Config>::default();

        let loader: Arc<dyn Loader> = Arc::new(TestLoader::default());
        app.on(&EventType::Load, AddLoaders::new(vec![loader]));

        app.load(None).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_app_load_custom_path() -> Result<()> {
        let app = Application::<Config>::default();

        let custom_path = PathBuf::from("config.toml");

        app.load(Some(custom_path)).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_app_init_success() -> Result<()> {
        let mut app = Application::<Config>::new(None);

        assert_eq!(app.config_tag, None);

        app = app.with_config_tag(&CONFIG);

        assert_eq!(app.config_tag, Some(&CONFIG));

        app.init().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_app_start_success() -> Result<()> {
        let app = Application::<Config>::default();

        app.start().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_app_stop_success() -> Result<()> {
        let app = Application::<Config>::default();

        app.stop().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_app_getconfig_success() -> Result<()> {
        let mut app = Application::<Config>::default();

        let config = app.get_config().await;
        assert!(config.is_err());

        app.inject(Config::default()).await?;

        let config = app.get_config().await;
        assert!(config.is_ok());

        app.remove::<Config>().await?;

        app = app.with_config_tag(&CONFIG);

        let config = app.get_config().await;
        assert!(config.is_err());

        app.inject_tag(&CONFIG, Config::default()).await?;

        let config = app.get_config().await;
        assert!(config.is_ok());

        Ok(())
    }
}
