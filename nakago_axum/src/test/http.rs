use std::sync::Arc;

use async_trait::async_trait;
use hyper::{client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;

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
