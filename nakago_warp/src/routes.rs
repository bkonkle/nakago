use std::{any::Any, sync::Arc};

use anyhow::anyhow;
use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};
use tokio::sync::Mutex;
use warp::{filters::BoxedFilter, http::Method, reply::Reply, wrap_fn, Filter};

/// A Route that will be nested within a higher-level Router, wrapped in a Mutex to safely move
pub type Route = BoxedFilter<(Box<dyn Reply>,)>;

/// A collection of Routes
pub type Routes = Mutex<Vec<Route>>;

/// A hook to initialize a particular route
pub struct Init<F>
where
    F: Fn(BoxedFilter<(Inject,)>) -> BoxedFilter<(Box<dyn Reply>,)> + Clone,
{
    path: String,
    handler: F,
    method: Method,
}

impl<F> Init<F>
where
    F: Fn(BoxedFilter<(Inject,)>) -> BoxedFilter<(Box<dyn Reply>,)> + Clone,
{
    /// Create a new Init instance
    pub fn new(method: Method, path: &str, handler: F) -> Self {
        Self {
            method,
            path: path.to_string(),
            handler,
        }
    }
}

#[async_trait]
impl<F> Hook for Init<F>
where
    F: Fn(BoxedFilter<(Inject,)>) -> BoxedFilter<(Box<dyn Reply>,)> + Send + Sync + Any + Clone,
{
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let method = match self.method {
            Method::HEAD => warp::head().boxed(),
            Method::GET => warp::get().boxed(),
            Method::OPTIONS => warp::options().boxed(),
            Method::PATCH => warp::patch().boxed(),
            Method::POST => warp::post().boxed(),
            Method::PUT => warp::put().boxed(),
            _ => {
                return Err(hooks::Error::Any(Arc::new(anyhow!(format!(
                    "Unsupported Route Method: {}",
                    self.method
                )))))
            }
        };

        let route = warp::path(self.path.clone())
            .and(method)
            .and(with_injection(i.clone()))
            .boxed()
            .with(wrap_fn(self.handler.clone()));

        if let Some(routes) = i.get_type_opt::<Routes>().await? {
            routes.lock().await.push(route);
        } else {
            i.inject_type::<Routes>(Mutex::new(vec![route])).await?;
        }

        Ok(())
    }
}

// Add the injection container to the handler
fn with_injection(i: Inject) -> BoxedFilter<(Inject,)> {
    let i = i.clone();

    warp::any().map(move || i.clone()).boxed()
}
