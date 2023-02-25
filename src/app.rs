use std::{
    any::Any,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use async_trait::async_trait;

use crate::{
    config::{self, Config},
    inject,
};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// A trait for async
#[async_trait]
pub trait LifecycleHook {
    /// Provide a dependency for the container
    async fn handle(&mut self, i: &mut inject::Inject) -> inject::Result<()>;
}

/// A no-op hook that does nothing, for use as a default
struct NoOpHook {}

#[async_trait]
impl LifecycleHook for NoOpHook {
    async fn handle(&mut self, _i: &mut inject::Inject) -> inject::Result<()> {
        Ok(())
    }
}

/// The top-level Application struct
pub struct Application<C: Config> {
    startup: Box<dyn LifecycleHook>,
    shutdown: Box<dyn LifecycleHook>,
    i: inject::Inject,
    _phantom: PhantomData<C>,
}

impl<C: Config> Default for Application<C> {
    fn default() -> Self {
        Self {
            startup: Box::new(NoOpHook {}),
            shutdown: Box::new(NoOpHook {}),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }
}

impl<C: Config> Application<C> {
    /// Create a new Application instance with a startup hook
    pub fn with_startup<H: LifecycleHook + 'static>(startup: H) -> Self {
        Self {
            startup: Box::new(startup),
            shutdown: Box::new(NoOpHook {}),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Create a new Application instance with a shutdown hook
    pub fn with_shutdown<H: LifecycleHook + 'static>(shutdown: H) -> Self {
        Self {
            startup: Box::new(NoOpHook {}),
            shutdown: Box::new(shutdown),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Create a new Application instance with a startup and shutdown hook
    pub fn with_hooks<H1: LifecycleHook + 'static, H2: LifecycleHook + 'static>(
        startup: H1,
        shutdown: H2,
    ) -> Self {
        Self {
            startup: Box::new(startup),
            shutdown: Box::new(shutdown),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }
}

impl<C> Deref for Application<C>
where
    C: Config,
{
    type Target = inject::Inject;

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
    /// Initialize the App
    ///
    /// **Provides:**
    ///   - `C: Config`
    pub async fn start(&mut self, config_path: Option<PathBuf>) -> inject::Result<()> {
        // Run the startup hook
        self.startup.handle(&mut self.i).await?;

        // Initialize the Config using the given path
        config::init::<C>(&mut self.i, config_path).await?;

        Ok(())
    }

    /// Shutdown the App
    pub async fn stop(&mut self) -> inject::Result<()> {
        // Run the shutdown hook
        self.shutdown.handle(&mut self.i).await?;

        Ok(())
    }
}
