use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use http::{Method, Request};
use hyper::{client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;
use serde_json::Value;

/// Utilities for testing HTTP GraphQL endpoints with Hyper
pub struct Http {
    base_url: String,
}

impl Http {
    /// Construct a new GraphQL helper with a path to the endpoint
    pub fn new(base_url: String) -> Self {
        Http { base_url }
    }

    /// Create a GraphQL query request for Hyper with an optional auth token
    pub fn call(&self, path: &str, body: Value, token: Option<&str>) -> Result<Request<Body>> {
        let base_url = if path.starts_with('/') {
            self.base_url.strip_suffix('/').unwrap_or(&self.base_url)
        } else {
            &self.base_url
        };

        let mut req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", base_url, path));

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let body = serde_json::to_string(&body)?;

        req.body(Body::from(body)).map_err(|err| err.into())
    }
}

/// A Tag for the Test HTTP Client
///   - Tag(AxumTestHttpClient)
pub const HTTP_CLIENT: Tag<Client<HttpsConnector<HttpConnector>>> = Tag::new("AxumTestHttpClient");

/// A Dependency Injection provider for a simple Test HTTP client using hyper
#[derive(Default)]
pub struct HttpClientProvider {}

#[Provider]
#[async_trait]
impl Provider<Client<HttpsConnector<HttpConnector>>> for HttpClientProvider {
    async fn provide(
        self: Arc<Self>,
        _i: Inject,
    ) -> InjectResult<Arc<Client<HttpsConnector<HttpConnector>>>> {
        Ok(Arc::new(
            Client::builder().build::<_, Body>(HttpsConnector::new()),
        ))
    }
}
