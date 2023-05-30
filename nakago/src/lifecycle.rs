use fnv::FnvHashMap;

use crate::inject::{self, hooks::Hooks, Hook};

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
    events: FnvHashMap<EventType, Hooks>,
}

impl Events {
    /// Set a new lifecycle hook that will fire on the given EventType
    pub fn on(&mut self, event: &EventType, hook: impl Hook) {
        if let Some(hooks) = self.events.get_mut(event) {
            hooks.push(hook);
        } else {
            self.events.insert(*event, Hooks::new(hook));
        }
    }

    /// Set a number of new lifecycle hooks that will fire on the given EventType
    pub fn when(&mut self, event: &EventType, hooks: Hooks) {
        if let Some(existing) = self.events.get_mut(event) {
            existing.extend(hooks);
        } else {
            self.events.insert(*event, hooks);
        }
    }

    /// Check if the given lifecycle event has any hooks
    pub fn has(&self, event: &EventType) -> bool {
        self.events.contains_key(event)
    }

    /// Trigger the given lifecycle event and handle hooks with the given injection container
    pub async fn trigger(
        &mut self,
        event: &EventType,
        i: &mut inject::Inject,
    ) -> inject::Result<()> {
        if let Some(hooks) = self.events.get(event) {
            for hook in hooks.iter() {
                hook.handle(i).await?;
            }
        }

        Ok(())
    }
}
