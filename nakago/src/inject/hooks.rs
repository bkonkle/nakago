use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

use futures::Future;

use super::{Inject, Result};

pub type PendingHook<'a> = Pin<Box<dyn Future<Output = Result<()>> + 'a>>;

/// A hook that can be run at various points in the lifecycle of an application
pub type Hook<'a> = dyn FnOnce(&'a mut Inject<'a>) -> PendingHook<'a>;

impl<'a> Inject<'a> {
    /// Handle a hook by running it against the Inject container
    pub async fn handle<F>(&'a mut self, hook: F) -> Result<()>
    where
        F: FnOnce(&'a mut Inject<'a>) -> PendingHook<'a>,
    {
        hook(self).await
    }
}

/// A collection of hooks with convenience methods for modifying it
#[derive(Default)]
pub struct Hooks<'a>(Vec<Box<Hook<'a>>>);

impl<'a> Hooks<'a> {
    /// Create a new collection of Hooks starting with the given Hook
    pub fn new<F>(hook: F) -> Self
    where
        F: FnOnce(&'a mut Inject<'a>) -> PendingHook<'a>,
    {
        Self(vec![Box::new(hook)])
    }

    /// Create a new collection of Hooks starting with the given Hooks
    pub fn from(hooks: Vec<Box<Hook<'a>>>) -> Self {
        Self(hooks)
    }

    /// Add a new hook to the collection
    pub fn push<F>(&mut self, hook: F)
    where
        F: FnOnce(&'a mut Inject<'a>) -> PendingHook<'a>,
    {
        self.0.push(Box::new(hook));
    }

    /// Add a number of new hooks to the collection
    pub fn extend(&mut self, hooks: Hooks<'a>) {
        self.0.extend(hooks.0);
    }

    /// Convenienve method to add a new hook to the collection, intended for chaining
    pub fn and(mut self, hook: impl FnOnce(&'a mut Inject<'a>) -> PendingHook<'a>) -> Self {
        self.push(hook);

        self
    }

    /// Handle all hooks in the collection
    pub async fn handle(&mut self, i: &'a mut Inject<'a>) -> Result<()> {
        for hook in self.0.drain(..) {
            hook(i).await?;
        }

        Ok(())
    }
}

impl<'a> Deref for Hooks<'a> {
    type Target = Vec<Box<Hook<'a>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Hooks<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Iterator for Hooks<'a> {
    type Item = Box<Hook<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
