use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::Route;

use super::{handlers, State};

/// Initialize the Health Route
pub fn new_health_route(_: Inject) -> Route<State> {
    Route::new("/", Router::new().route("/health", get(handlers::health)))
}

/// Initialize the User route
pub fn new_user_route(_: Inject) -> Route<State> {
    Route::new(
        "/",
        Router::new().route("/username", get(handlers::get_username)),
    )
}
