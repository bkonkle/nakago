use std::ops::{Deref, DerefMut};

use nakago::Inject;

#[derive(Clone)]
/// Axum State used to carry the injection container
pub struct State {
    i: Inject,
}

impl State {
    /// Create a new State instance
    pub fn new(i: Inject) -> Self {
        Self { i }
    }
}

impl Deref for State {
    type Target = Inject;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.i
    }
}
