use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

use futures::Future;

use super::{Inject, Result};

/// A hook that can be run at various points in the lifecycle of an application
pub type Hook = dyn FnOnce(&mut Inject) -> Pin<Box<dyn Future<Output = Result<()>>>>;

impl Inject {
    /// Handle a hook by running it against the Inject container
    pub async fn handle<F>(&mut self, hook: F) -> Result<()>
    where
        F: FnOnce(&mut Inject) -> Pin<Box<dyn Future<Output = Result<()>>>>,
    {
        hook(self).await
    }
}

/// A collection of hooks with convenience methods for modifying it
#[derive(Default)]
pub struct Hooks(Vec<Box<Hook>>);

impl Hooks {
    /// Create a new collection of Hooks starting with the given Hook
    pub fn new<F>(hook: F) -> Self
    where
        F: FnOnce(&mut Inject) -> Pin<Box<dyn Future<Output = Result<()>>>>,
    {
        Self(vec![Box::new(hook)])
    }

    /// Create a new collection of Hooks starting with the given Hooks
    pub fn from(hooks: Vec<Box<Hook>>) -> Self {
        Self(hooks)
    }

    /// Add a new hook to the collection
    pub fn push<F>(&mut self, hook: F)
    where
        F: FnOnce(&mut Inject) -> Pin<Box<dyn Future<Output = Result<()>>>>,
    {
        self.0.push(Box::new(hook));
    }

    /// Add a number of new hooks to the collection
    pub fn extend(&mut self, hooks: Hooks) {
        self.0.extend(hooks.0);
    }

    /// Convenienve method to add a new hook to the collection, intended for chaining
    pub fn and(
        mut self,
        hook: impl FnOnce(&mut Inject) -> Pin<Box<dyn Future<Output = Result<()>>>>,
    ) -> Self {
        self.push(hook);

        self
    }

    /// Handle all hooks in the collection
    pub async fn handle(self, i: &mut Inject) -> Result<()> {
        for hook in self.0 {
            let temp = hook(i).await?;
        }

        Ok(())
    }
}

impl Deref for Hooks {
    type Target = Vec<Box<Hook>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Hooks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Iterator for Hooks {
    type Item = Box<Hook>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
