use std::{
    any::Any,
    convert::Infallible,
    ops::{Deref, DerefMut},
    sync::Arc,
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

/// An Axum extractor to inject dependencies from Nakago
#[derive(new)]
pub struct Inject<T>(pub Arc<T>);

impl<T> Deref for Inject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T: Send + Sync + Any> FromRequestParts<State> for Inject<T> {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &State,
    ) -> Result<Self, Self::Rejection> {
        let t = state.get::<T>().await.unwrap();

        Ok(Self::new(t))
    }
}
