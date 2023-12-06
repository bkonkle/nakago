use std::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use derive_new::new;

#[derive(Clone, new)]
/// Axum State used to carry the injection container
pub struct State {
    i: nakago::Inject,
}

impl Deref for State {
    type Target = nakago::Inject;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.i
    }
}

/// An extractor for the injection container
pub struct Inject(pub nakago::Inject);

#[async_trait]
impl FromRequestParts<State> for Inject {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &State,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(state.i.clone()))
    }
}

impl Deref for Inject {
    type Target = nakago::Inject;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Inject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
