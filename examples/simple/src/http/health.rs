use std::sync::Arc;

use async_trait::async_trait;
use axum::{routing::get, Json, Router};
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::Route;
use nakago_derive::Provider;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

/// Tag(health::Route)
pub const ROUTE: Tag<Route> = Tag::new("health::Route");

/// A Health Check Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// The Status code
    code: usize,

    /// Whether the check was successful or not
    success: bool,
}

#[derive(Clone, Default)]
pub struct Controller {}

impl Controller {
    /// Handle health check requests
    pub async fn handle(self) -> Json<HealthResponse> {
        Json(HealthResponse {
            code: 200,
            success: true,
        })
    }
}

#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Route> for Provide {
    async fn provide(self: Arc<Self>, _: Inject) -> inject::Result<Arc<Route>> {
        let controller = Controller::default();

        let route: Router<HealthResponse> =
            Router::new().route("/health", get(|| async move { controller.handle().await }));

        Ok(Arc::new(Mutex::new(route)))
    }
}
