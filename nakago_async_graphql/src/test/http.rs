use std::{ops::Deref, sync::Arc};

use nakago_axum::test::http::Http;
use reqwest::{Method, RequestBuilder};
use serde_json::{json, Value};

/// Utilities for testing HTTP GraphQL endpoints with Hyper
pub struct GraphQL {
    /// The HTTP utils instance
    pub http: Arc<Http>,

    endpoint: String,
}

impl GraphQL {
    /// Create a new instance of the `GraphQL` struct with a base URL for requests
    pub fn new(http: Arc<Http>, endpoint: String) -> Self {
        GraphQL { http, endpoint }
    }

    /// Create a GraphQL query request for Hyper with an optional auth token
    pub fn query(&self, query: &str, variables: Value, token: Option<&str>) -> RequestBuilder {
        let json = json!({ "query": query, "variables": variables });

        self.http
            .request_json(Method::POST, &self.endpoint, json, token)
    }
}

impl Deref for GraphQL {
    type Target = Http;

    fn deref(&self) -> &Self::Target {
        &self.http
    }
}
