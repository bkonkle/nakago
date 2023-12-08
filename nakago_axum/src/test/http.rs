use std::ops::{Deref, DerefMut};

use reqwest::{Client, Method, RequestBuilder};
use serde_json::Value;

/// Utilities for testing HTTP endpoints with Hyper
pub struct Http {
    /// The HTTP client instance
    pub client: Client,

    base_url: String,
}

impl Http {
    /// Create a new instance of the `Http` struct with a base URL for requests
    pub fn new(base_url: String) -> Self {
        Http {
            client: Client::new(),
            base_url,
        }
    }

    /// Create an HTTP GET request with an optional auth token
    pub fn get_json(&self, path: &str, token: Option<&str>) -> RequestBuilder {
        let mut req = self.client.get(self.get_url(path));

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        req
    }

    /// Create an HTTP request for Hyper with an optional auth token
    pub fn request_json(
        &self,
        method: Method,
        path: &str,
        json: Value,
        token: Option<&str>,
    ) -> RequestBuilder {
        let mut req = self.client.request(method, self.get_url(path));

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        req.body(serde_json::to_string(&json).expect("Unable to serialize body"))
    }

    /// Return the given path with the base URL prepended
    pub fn get_url(&self, path: &str) -> String {
        let base_url = if path.starts_with('/') {
            self.base_url.strip_suffix('/').unwrap_or(&self.base_url)
        } else {
            &self.base_url
        };

        format!("{}{}", base_url, path)
    }
}

impl Deref for Http {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for Http {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
