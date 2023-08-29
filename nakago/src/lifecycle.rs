use fnv::FnvHashMap;

use crate::{
    inject::{Hook, Inject},
    InjectResult,
};

/// Lifecycle Event Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    /// The Application is loading dependencies and configuration. During this phase, the
    /// Application should provide any dependencies or config loaders that are necessary to
    /// initialize and start the App.
    Load,

    /// The Application has initialized dependencies and configuration. During this phase, the
    /// Application should perform any initialization steps and construct anything necessary to
    /// start the App.
    Init,

    /// The Application has started up and is now running. During this phase, the Application
    /// should start any background tasks or other long-running processes necessary to keep the App
    /// running.
    Startup,

    /// The Application is shutting down. During this phase, the Application should perform any
    /// cleanup necessary to cleanly stop the App.
    Shutdown,
}

/// Events is a collection of lifecycle hooks that can be added to or triggered
#[derive(Default)]
pub struct Events {
    hooks: FnvHashMap<EventType, Vec<Box<dyn Hook>>>,
}

impl Events {
    /// Set a new lifecycle hook that will fire on the given EventType
    pub fn on(&mut self, event: &EventType, hook: impl Hook) {
        if let Some(hooks) = self.hooks.get_mut(event) {
            hooks.push(Box::new(hook));
        } else {
            self.hooks.insert(*event, vec![Box::new(hook)]);
        }
    }

    /// Trigger the given lifecycle event and handle hooks with the given injection container
    pub async fn trigger(&self, event: &EventType, i: Inject) -> InjectResult<()> {
        if let Some(hooks) = self.hooks.get(event) {
            for hook in hooks {
                hook.handle(i.clone()).await?;
            }
        }

        Ok(())
    }
}
