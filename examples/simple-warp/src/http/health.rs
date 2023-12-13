use std::convert::Infallible;

use nakago_warp::Route;
use serde::{Deserialize, Serialize};
use warp::{reply::Reply, Filter};

/// A Health Check Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// The Status code
    code: usize,

    /// Whether the check was successful or not
    success: bool,
}

impl Reply for HealthResponse {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status(warp::reply::json(&self), warp::http::StatusCode::OK)
            .into_response()
    }
}

/// Create a Health Check Route
pub fn health_check() -> Route {
    warp::path("health")
        .and(warp::get())
        .and_then(health_handler)
        .map(|a| Box::new(a) as Box<dyn Reply>)
        .boxed()
}

/// Handle Health Check requests
pub async fn health_handler() -> Result<HealthResponse, Infallible> {
    Ok(HealthResponse {
        code: 200,
        success: true,
    })
}
