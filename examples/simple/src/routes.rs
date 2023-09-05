use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::FromRef, routing::get, Router};
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_axum::{
    app::State,
    auth::{authenticate::AuthState, AUTH_STATE},
    Route,
};
use nakago_derive::Provider;

use crate::users::service::USERS_SERVICE;

use super::handlers::{get_user_handler, health_handler, update_user_handler, UserState};

/// Tag(AppState)
pub const STATE: Tag<AppState> = Tag::new("AppState");

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct AppState {
    auth: AuthState,
    users: UserState,
}

impl AppState {
    /// Create a new AppState instance
    pub fn new(auth: AuthState, users: UserState) -> Self {
        Self { auth, users }
    }
}

impl State for AppState {}

/// Initialize the Health Route
pub fn new_health_route(_: Inject) -> Route<AppState> {
    Route::new("/", Router::new().route("/health", get(health_handler)))
}

/// Initialize the User Route
pub fn new_user_route(_: Inject) -> Route<AppState> {
    Route::new(
        "/",
        Router::new().route("/user", get(get_user_handler).post(update_user_handler)),
    )
}

/// Provide the AppState for Axum
///
/// **Provides:** `AppState`
///
/// **Depends on:**
///   - `Tag(AuthState)`
///   - `Tag(UsersService)`
#[derive(Default)]
pub struct ProvideAppState {}

#[Provider]
#[async_trait]
impl Provider<AppState> for ProvideAppState {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<AppState>> {
        let auth = i.get(&AUTH_STATE).await?;
        let users = i.get(&USERS_SERVICE).await?;
        let user_state = UserState::new(users);

        Ok(Arc::new(AppState::new((*auth).clone(), user_state)))
    }
}
