use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::PathBuf,
    pin::Pin,
    sync::Arc,
};

use futures::Future;

use crate::{
    config::loader::provide_config, inject, lifecycle::Events, log::init_rust_log,
    panic::init_handle_panic, Config, EventType, Hooks,
};

/// The top-level Application struct
pub struct Application<C: Config> {
    events: Events,
    i: inject::Inject,
    _phantom: PhantomData<C>,
}

impl<C: Config> Default for Application<C> {
    fn default() -> Self {
        Self {
            events: Events::default(),
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
    /// Set a new lifecycle hook that will fire on the given EventType
    pub fn on<F>(&mut self, event: &EventType, hook: F)
    where
        F: for<'a> FnOnce(
            &mut inject::Inject<'a>,
        ) -> Pin<Box<dyn Future<Output = inject::Result<()>>>>,
    {
        self.events.on(event, hook);
    }

    /// Set a number of new lifecycle hooks that will fire on the given EventType
    pub fn when(&mut self, event: &EventType, hooks: Hooks<'_>) {
        self.events.when(event, hooks);
    }

    /// Initialize the App. This culminates in a loaded Config which is injected.
    ///
    /// **Provides:**
    ///   - `C: Config`
    pub async fn init(&mut self, config_path: Option<PathBuf>) -> inject::Result<()> {
        // Check to see if there are any hooks for the Init event
        if !self.events.has(&EventType::Init) {
            // If not, add the default hooks
            self.events.when(&EventType::Init, default_init_hooks());
        };

        // Trigger the Init lifecycle event
        self.events.trigger(&EventType::Init, &mut self.i).await?;

        // Initialize the Config using the given path
        self.i
            .provide_type::<Arc<C>, _>(provide_config::<C>(config_path))?;

        Ok(())
    }

    /// Run the Application by starting the listener
    pub async fn start(&mut self) -> inject::Result<()> {
        // Trigger the Start lifecycle event
        self.events
            .trigger(&EventType::Startup, &mut self.i)
            .await?;

        Ok(())
    }

    /// Shut down the Application by stopping the listener
    pub async fn stop(&mut self) -> inject::Result<()> {
        // Trigger the Stop lifecycle event
        self.events
            .trigger(&EventType::Shutdown, &mut self.i)
            .await?;

        Ok(())
    }
}

/// The default lifecycle hooks that will be run on the Init event
pub fn default_init_hooks<'a>() -> Hooks<'a> {
    Hooks::from(vec![Box::new(init_handle_panic), Box::new(init_rust_log)])
}
