use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::{self, auth};
use nakago_derive::Provider;

/// Tag(app::State)
pub const STATE: Tag<State> = Tag::new("app::State");

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct State {
    auth: auth::State,
}

impl nakago_axum::State for State {}

/// Provide the State for Axum
///
/// **Provides:** `app::State`
///
/// **Depends on:**
///   - `Tag(auth::State)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<State> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<State>> {
        let auth = i.get(&auth::STATE).await?;

        Ok(Arc::new(State {
            auth: (*auth).clone(),
        }))
    }
}
