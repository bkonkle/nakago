use std::{any::Any, pin::Pin, result::Result, sync::Arc};

use anyhow::anyhow;
use async_trait::async_trait;
use futures_util::Future;
use nakago::{hooks, Hook, Inject};
use tokio::sync::Mutex;
use warp::{filters::BoxedFilter, http::Method, reject::Rejection, reply::Reply, Filter};

/// A Route that will be nested within a higher-level Router, wrapped in a Mutex to safely move
pub type Route = BoxedFilter<(Box<dyn Reply>,)>;

/// A collection of Routes
pub type Routes = Mutex<Vec<Route>>;

type Handler<T> = Pin<Box<dyn Future<Output = Result<T, Rejection>> + Send>>;

/// A hook to initialize a particular route
pub struct Init<T, F>
where
    F: Fn(Inject) -> Handler<T> + Send + Sync + Any,
    T: Reply + Send + Sync + Any,
{
    path: String,
    handler: F,
    method: Method,
}

impl<T, F> Init<T, F>
where
    F: Fn(Inject) -> Handler<T> + Send + Sync + Any,
    T: Reply + Send + Sync + Any,
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
impl<T, F> Hook for Init<T, F>
where
    F: Fn(Inject) -> Handler<T> + Send + Sync + Any + Clone,
    T: Reply + Send + Sync + Any,
{
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let router = match self.method {
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
            .and(router)
            .and(with_injection(i.clone()))
            .and_then(self.handler.clone())
            .map(|a| Box::new(a) as Box<dyn Reply>)
            .boxed();

        if let Some(routes) = i.get_type_opt::<Routes>().await? {
            routes.lock().await.push(route);
        } else {
            i.inject_type::<Routes>(Mutex::new(vec![route])).await?;
        }

        Ok(())
    }
}

// Add the injection container to the handler
fn with_injection(
    i: Inject,
) -> impl Filter<Extract = (Inject,), Error = std::convert::Infallible> + Clone {
    let i = i.clone();

    warp::any().map(move || i.clone())
}
