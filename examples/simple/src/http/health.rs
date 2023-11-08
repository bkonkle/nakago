use std::sync::Arc;

use async_trait::async_trait;
use axum::{routing::get, Json, Router};
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::Route;
use nakago_derive::Provider;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

/// Tag(health::check::Route)
pub const CHECK_ROUTE: Tag<Route> = Tag::new("health::check::Route");

/// A Health Check Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// The Status code
    code: usize,

    /// Whether the check was successful or not
    success: bool,
}

/// Handle health check requests
pub async fn health_check() -> Json<HealthResponse> {
    println!(">------ HEALTH CHECK ------<");

    Json(HealthResponse {
        code: 200,
        success: true,
    })
}

/// A Provider for the health check route
#[derive(Default)]
pub struct ProvideCheck {}

#[Provider]
#[async_trait]
impl Provider<Route> for ProvideCheck {
    async fn provide(self: Arc<Self>, _: Inject) -> inject::Result<Arc<Route>> {
        let route = Router::new().route("/health", get(health_check));

        Ok(Arc::new(Mutex::new(route)))
    }
}
