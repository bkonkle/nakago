use std::marker::PhantomData;

use async_trait::async_trait;
use axum::Router;
use hyper::Body;
use nakago::{Hook, Inject, InjectResult};
use tokio::sync::Mutex;

use crate::app::State;

/// A Route that will be nested within a higher-level Router
pub struct Route<S = (), B = Body> {
    pub(crate) path: String,
    pub(crate) router: Mutex<Router<S, B>>,
}

impl<S, B> Route<S, B> {
    /// Create a new Route
    pub fn new(path: &str, router: Router<S, B>) -> Self {
        Self {
            path: path.to_string(),
            router: Mutex::new(router),
        }
    }
}

/// A hook to initialize a particular route
pub struct InitRoute<S: State> {
    get_route: fn(Inject) -> Route<S>,
    _phantom: PhantomData<S>,
}

impl<S: State> InitRoute<S> {
    /// Create a new InitRoute instance
    pub fn new(get_route: fn(Inject) -> Route<S>) -> Self {
        Self {
            get_route,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<S: State> Hook for InitRoute<S> {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let route = (self.get_route)(i.clone());

        if let Some(routes) = i.get_type_opt::<Mutex<Vec<Route<S>>>>().await? {
            routes.lock().await.push(route);
        } else {
            i.inject_type(Mutex::new(vec![route])).await?;
        }

        Ok(())
    }
}
