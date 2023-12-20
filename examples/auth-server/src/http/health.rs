use axum::Json;
use serde::{Deserialize, Serialize};

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
    Json(HealthResponse {
        code: 200,
        success: true,
    })
}
