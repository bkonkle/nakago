use std::{any::Any, marker::PhantomData, sync::Arc};

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{
    handler::Handler,
    routing::{get, head, options, patch, post, put, trace},
    Router,
};
use hyper::Method;
use nakago::{inject, Hook, Inject};
use tokio::sync::Mutex;

use crate::State;

/// A Route that will be nested within a higher-level Router, wrapped in a Mutex to safely move
pub type Route = Mutex<Router<State>>;

/// A collection of Routes
pub type Routes = Mutex<Vec<Route>>;

/// A hook to initialize a particular route
pub struct Init<H, T> {
    path: String,
    handler: H,
    method: Method,
    _phantom: PhantomData<T>,
}

impl<H, T> Init<H, T> {
    /// Create a new Init instance
    pub fn new(method: Method, path: &str, handler: H) -> Self {
        Self {
            method,
            path: path.to_string(),
            handler,
            _phantom: Default::default(),
        }
    }
}

#[async_trait]
impl<H, T> Hook for Init<H, T>
where
    T: Send + Sync + Any,
    H: Handler<T, State> + Send + Sync,
{
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let router = match self.method {
            Method::HEAD => head(self.handler.clone()),
            Method::GET => get(self.handler.clone()),
            Method::OPTIONS => options(self.handler.clone()),
            Method::PATCH => patch(self.handler.clone()),
            Method::POST => post(self.handler.clone()),
            Method::PUT => put(self.handler.clone()),
            Method::TRACE => trace(self.handler.clone()),
            _ => {
                return Err(inject::Error::Provider(Arc::new(anyhow!(format!(
                    "Unsupported Route Method: {}",
                    self.method
                )))))
            }
        };

        let route = Router::new().route(&self.path, router);

        if let Some(routes) = i.get_type_opt::<Routes>().await? {
            routes.lock().await.push(Mutex::new(route));
        } else {
            i.inject_type::<Routes>(Mutex::new(vec![Mutex::new(route)]))
                .await?;
        }

        Ok(())
    }
}
