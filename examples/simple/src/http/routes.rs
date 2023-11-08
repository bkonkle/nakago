use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::Route;

use super::{health, user};

/// Initialize the Health Route
pub fn new_health_route(_: Inject) -> Route {
    Route::new("/", Router::new().route("/health", get(health::health)))
}

/// Initialize the User route
pub fn new_user_route(_: Inject) -> Route {
    Route::new(
        "/",
        Router::new().route("/username", get(health::get_username)),
    )
}
