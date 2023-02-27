use async_trait::async_trait;
use backtrace::Backtrace;
use crossterm::{execute, style::Print};
use std::{
    any::Any,
    io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    panic::{self, PanicInfo},
    path::PathBuf,
};
use tracing_subscriber::prelude::*;

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
pub struct NoOpHook {}

#[async_trait]
impl LifecycleHook for NoOpHook {
    async fn handle(&mut self, _i: &mut inject::Inject) -> inject::Result<()> {
        Ok(())
    }
}

/// The top-level Application struct
pub struct Application<C: Config> {
    init: Box<dyn LifecycleHook + Send>,
    startup: Box<dyn LifecycleHook + Send>,
    i: inject::Inject,
    _phantom: PhantomData<C>,
}

impl<C: Config> Default for Application<C> {
    fn default() -> Self {
        Self {
            init: Box::new(NoOpHook {}),
            startup: Box::new(NoOpHook {}),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }
}

impl<C: Config> Application<C> {
    /// Create a new Application instance with a startup and shutdown hook
    pub fn with_hooks<H1: LifecycleHook + Send + 'static, H2: LifecycleHook + Send + 'static>(
        init: H1,
        startup: H2,
    ) -> Self {
        Self {
            init: Box::new(init),
            startup: Box::new(startup),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Create a new Application instance with an init hook
    pub fn with_init<H: LifecycleHook + Send + 'static>(init: H) -> Self {
        Self {
            init: Box::new(init),
            startup: Box::new(NoOpHook {}),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Create a new Application instance with a startup hook
    pub fn with_startup<H: LifecycleHook + Send + 'static>(startup: H) -> Self {
        Self {
            init: Box::new(NoOpHook {}),
            startup: Box::new(startup),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Set the init hook while building the Application
    pub fn and_init<H: LifecycleHook + Send + 'static>(self, init: H) -> Self {
        Self {
            init: Box::new(init),
            ..self
        }
    }

    /// Set the startup hook while building the Application
    pub fn and_startup<H: LifecycleHook + Send + 'static>(self, startup: H) -> Self {
        Self {
            startup: Box::new(startup),
            ..self
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
    pub async fn init(&mut self, config_path: Option<PathBuf>) -> inject::Result<()> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        // Process setup
        panic::set_hook(Box::new(handle_panic));

        // Run the init hook
        self.init.handle(&mut self.i).await?;

        // Initialize the Config using the given path
        config::init::<C>(&mut self.i, config_path).await?;

        Ok(())
    }

    /// Start the App
    pub async fn start(&mut self) -> inject::Result<()> {
        // Run the startup hook
        self.startup.handle(&mut self.i).await?;

        Ok(())
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
