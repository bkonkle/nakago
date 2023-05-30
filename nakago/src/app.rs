use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use crate::{
    lifecycle::Events, log::InitRustLog, panic::InitHandlePanic, Config, ConfigProvider, EventType,
    Hook, Hooks, Inject, InjectResult,
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

    /// Set a number of new lifecycle hooks that will fire on the given EventType
    pub fn when(&mut self, event: &EventType, hooks: Hooks) {
        self.events.when(event, hooks);
    }

    /// Initialize the App. This culminates in a loaded Config which is injected.
    ///
    /// **Provides:**
    ///   - `C: Config`
    pub async fn init(&mut self, config_path: Option<PathBuf>) -> InjectResult<()> {
        // Check to see if there are any hooks for the Init event
        if !self.events.has(&EventType::Init) {
            // If not, add the default hooks
            self.events.when(&EventType::Init, default_init_hooks());
        };

        // Trigger the Init lifecycle event
        self.events.trigger(&EventType::Init, &mut self.i).await?;

        // Initialize the Config using the given path
        self.i
            .provide_type(ConfigProvider::<C>::new(config_path))
            .await?;

        Ok(())
    }

    /// Run the Application by starting the listener
    pub async fn start(&mut self) -> InjectResult<()> {
        // Trigger the Start lifecycle event
        self.events
            .trigger(&EventType::Startup, &mut self.i)
            .await?;

        Ok(())
    }

    /// Shut down the Application by stopping the listener
    pub async fn stop(&mut self) -> InjectResult<()> {
        // Trigger the Stop lifecycle event
        self.events
            .trigger(&EventType::Shutdown, &mut self.i)
            .await?;

        Ok(())
    }
}

/// The default lifecycle hooks that will be run on the Init event
pub fn default_init_hooks() -> Hooks {
    Hooks::from(vec![
        Box::<InitHandlePanic>::default(),
        Box::<InitRustLog>::default(),
    ])
}
