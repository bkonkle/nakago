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
    config,
    inject::{self, Hook},
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
    pub async fn trigger(&mut self, event: &EventType) -> inject::Result<()> {
        self.events.trigger(event, self.i.clone()).await
    }

    /// Load the App's dependencies and configuration. Triggers the Load lifecycle event.
    pub async fn load(&self, config_path: Option<PathBuf>) -> inject::Result<()> {
        // Trigger the Load lifecycle event
        self.events
            .trigger(&EventType::Load, self.i.clone())
            .await?;

        // Load the Config using the given path
        self.i
            .handle(config::Init::new(config_path, self.config_tag))
            .await?;

        Ok(())
    }

    /// Initialize the App and provide the top-level Config. Triggers the Init lifecycle event.
    pub async fn init(&self) -> inject::Result<()> {
        // Trigger the Init lifecycle event
        self.events
            .trigger(&EventType::Init, self.i.clone())
            .await?;

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
    pub async fn start(&self) -> inject::Result<()> {
        self.events
            .trigger(&EventType::Startup, self.i.clone())
            .await?;

        Ok(())
    }

    /// Trigger the Shutdown lifecycle event.
    pub async fn stop(&self) -> inject::Result<()> {
        self.events
            .trigger(&EventType::Shutdown, self.i.clone())
            .await?;

        Ok(())
    }

    /// Get the top-level Config by tag or type
    pub async fn get_config(&self) -> inject::Result<Arc<C>> {
        let config = if let Some(tag) = self.config_tag {
            self.i.get(tag).await?
        } else {
            self.i.get_type::<C>().await?
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
