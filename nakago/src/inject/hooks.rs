use async_trait::async_trait;
use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use super::{Inject, Result};

/// A hook that can be run at various points in the lifecycle of an application
#[async_trait]
pub trait Hook: Any + Send {
    /// Handle the event by operating on the Inject container
    async fn handle(&self, i: &mut Inject) -> Result<()>;
}

impl Inject {
    /// Handle a hook by running it against the Inject container
    pub async fn handle<H: Hook>(&mut self, hook: H) -> Result<()> {
        hook.handle(self).await
    }
}

/// A collection of hooks with convenience methods for modifying it
#[derive(Default)]
pub struct Hooks(Vec<Box<dyn Hook>>);

impl Hooks {
    /// Create a new collection of Hooks starting with the given Hook
    pub fn new<H: Hook>(hook: H) -> Self {
        Self(vec![Box::new(hook)])
    }

    /// Create a new collection of Hooks starting with the given Hooks
    pub fn from(hooks: Vec<Box<dyn Hook>>) -> Self {
        Self(hooks)
    }

    /// Add a new hook to the collection
    pub fn push<H: Hook>(&mut self, hook: H) {
        self.0.push(Box::new(hook));
    }

    /// Add a number of new hooks to the collection
    pub fn extend(&mut self, hooks: Hooks) {
        self.0.extend(hooks.0);
    }

    /// Convenienve method to add a new hook to the collection, intended for chaining
    pub fn and(mut self, hook: impl Hook) -> Self {
        self.push(hook);

        self
    }

    /// Handle all hooks in the collection
    pub async fn handle(&self, i: &mut Inject) -> Result<()> {
        for hook in &self.0 {
            hook.handle(i).await?;
        }

        Ok(())
    }
}

impl Deref for Hooks {
    type Target = Vec<Box<dyn Hook>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Hooks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
