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
    inject::{Hook, Inject},
    lifecycle::Events,
    EventType, InjectResult,
};

/// The top-level Application struct
pub struct Application<C: Config> {
    events: Events,
    i: Inject,
    _phantom: PhantomData<C>,
}

impl<C: Config> Default for Application<C> {
    fn default() -> Self {
        Self {
            events: Events::default(),
            i: Inject::default(),
            _phantom: PhantomData,
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
    pub async fn trigger(&mut self, event: &EventType) -> InjectResult<()> {
        self.events.trigger(event, self.i.clone()).await
    }

    /// Initialize the App
    ///
    /// **Provides:**
    ///   - `C: Config`
    pub async fn init(&self, config_path: Option<PathBuf>) -> InjectResult<()> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        // Process setup
        panic::set_hook(Box::new(handle_panic));

        // Trigger the Init lifecycle event
        self.events
            .trigger(&EventType::Init, self.i.clone())
            .await?;

        // Initialize the Config using the given path
        InitConfig::<C>::new(config_path)
            .handle(self.i.clone())
            .await?;

        Ok(())
    }

    /// Run the Application by starting the listener
    pub async fn start(&self) -> InjectResult<()> {
        // Trigger the Start lifecycle event
        self.events
            .trigger(&EventType::Startup, self.i.clone())
            .await?;

        Ok(())
    }

    /// Shut down the Application by stopping the listener
    pub async fn stop(&self) -> InjectResult<()> {
        // Trigger the Stop lifecycle event
        self.events
            .trigger(&EventType::Shutdown, self.i.clone())
            .await?;

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
