use fnv::FnvHashMap;

use crate::inject::{self, Hook};

/// Lifecycle Event Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    /// The application is initializing
    Init,

    /// The application is starting up
    Startup,

    /// The application is shutting down
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
    pub async fn trigger(
        &mut self,
        event: &EventType,
        i: &mut inject::Inject,
    ) -> inject::Result<()> {
        if let Some(hooks) = self.hooks.get(event) {
            for hook in hooks {
                hook.handle(i).await?;
            }
        }

        Ok(())
    }
}
