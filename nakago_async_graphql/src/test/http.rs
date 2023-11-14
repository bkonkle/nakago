use anyhow::Result;
use axum::{
    body::Body,
    http::{Method, Request},
};
use derive_new::new;
use serde_json::{json, Value};

/// Utilities for testing HTTP GraphQL endpoints with Hyper
#[derive(new)]
pub struct GraphQL {
    url: String,
}

impl GraphQL {
    /// Create a GraphQL query request for Hyper with an optional auth token
    pub fn query(
        &self,
        query: &str,
        variables: Value,
        token: Option<&str>,
    ) -> Result<Request<Body>> {
        let mut req = Request::builder().method(Method::POST).uri(&self.url);

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let body = serde_json::to_string(&json!({ "query": query, "variables": variables }))?;

        req.body(Body::from(body)).map_err(|err| err.into())
    }
}
