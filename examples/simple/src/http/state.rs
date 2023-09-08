use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_axum::{
    app::State,
    auth::{authenticate::AuthState, AUTH_STATE},
};
use nakago_derive::Provider;

/// Tag(AppState)
pub const STATE: Tag<AppState> = Tag::new("AppState");

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct AppState {
    auth: AuthState,
}

impl State for AppState {}

/// Provide the AppState for Axum
///
/// **Provides:** `AppState`
///
/// **Depends on:**
///   - `Tag(AuthState)`
#[derive(Default)]
pub struct ProvideAppState {}

#[Provider]
#[async_trait]
impl Provider<AppState> for ProvideAppState {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<AppState>> {
        let auth = i.get(&AUTH_STATE).await?;

        Ok(Arc::new(AppState {
            auth: (*auth).clone(),
        }))
    }
}
