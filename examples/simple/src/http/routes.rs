use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::Route;

use super::{handlers::health_handler, state::AppState};

/// Initialize the Health Route
pub fn new_health_route(_: Inject) -> Route<AppState> {
    Route::new("/", Router::new().route("/health", get(health_handler)))
}
