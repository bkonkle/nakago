use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Json,
};
use nakago_axum::auth::Subject;
use serde::{Deserialize, Serialize};

use crate::{domains::users, graphql};

/// A Health Check Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// The Status code
    code: usize,

    /// Whether the check was successful or not
    success: bool,
}

#[derive(Clone)]
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
