use backtrace::Backtrace;
use crossterm::{execute, style::Print};
use std::{
    io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    panic::{self, PanicInfo},
    path::PathBuf,
};
use tracing_subscriber::prelude::*;

use crate::{
    config::{Config, InitConfig},
    inject::{self, Hook},
};

/// The top-level Application struct
pub struct Application<C: Config> {
    init: Box<dyn Hook>,
    startup: Box<dyn Hook>,
    i: inject::Inject,
    _phantom: PhantomData<C>,
}

impl<C: Config> Default for Application<C> {
    fn default() -> Self {
        Self {
            init: Box::new(inject::NoOpHook {}),
            startup: Box::new(inject::NoOpHook {}),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }
}

impl<C: Config> Application<C> {
    /// Create a new Application instance with a startup and shutdown hook
    pub fn with_hooks<H1: Hook, H2: Hook>(init: H1, startup: H2) -> Self {
        Self {
            init: Box::new(init),
            startup: Box::new(startup),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Create a new Application instance with an init hook
    pub fn with_init<H: Hook>(init: H) -> Self {
        Self {
            init: Box::new(init),
            startup: Box::new(inject::NoOpHook {}),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Create a new Application instance with a startup hook
    pub fn with_startup<H: Hook>(startup: H) -> Self {
        Self {
            init: Box::new(inject::NoOpHook {}),
            startup: Box::new(startup),
            i: inject::Inject::default(),
            _phantom: PhantomData,
        }
    }

    /// Set the init hook while building the Application
    pub fn and_init<H: Hook>(self, init: H) -> Self {
        Self {
            init: Box::new(init),
            ..self
        }
    }

    /// Set the startup hook while building the Application
    pub fn and_startup<H: Hook>(self, startup: H) -> Self {
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
        InitConfig::<C>::new(config_path)
            .handle(&mut self.i)
            .await?;

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
