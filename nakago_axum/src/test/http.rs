use anyhow::Result;
use axum::body::Body;
use derive_new::new;
use hyper::{Method, Request};
use serde_json::Value;

/// Utilities for testing HTTP endpoints with Hyper
#[derive(new)]
pub struct Http {
    base_url: String,
}

impl Http {
    /// Create an HTTP request for Hyper with an optional auth token
    pub fn call(
        &self,
        method: Method,
        path: &str,
        body: Value,
        token: Option<&str>,
    ) -> Result<Request<Body>> {
        let base_url = if path.starts_with('/') {
            self.base_url.strip_suffix('/').unwrap_or(&self.base_url)
        } else {
            &self.base_url
        };

        let mut req = Request::builder()
            .method(method)
            .uri(format!("{}{}", base_url, path));

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let body = serde_json::to_string(&body)?;

        req.body(Body::from(body)).map_err(|err| err.into())
    }
}
