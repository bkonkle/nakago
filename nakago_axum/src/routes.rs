use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use hyper::Body;
use nakago::{inject, Hook, Inject, Tag};
use tokio::sync::Mutex;

use crate::State;

/// A Route that will be nested within a higher-level Router
pub type Route<B = Body> = Mutex<Router<B>>;

/// A hook to initialize a particular route
pub struct Init {
    tag: &'static Tag<Route<State>>,
}

impl Init {
    /// Create a new Init hook for a Route
    pub fn new(tag: &'static Tag<Route<State>>) -> Self {
        Self { tag }
    }
}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let route = i.get(self.tag).await?;

        if let Some(routes) = i.get_type_opt::<Mutex<Vec<Arc<Route<State>>>>>().await? {
            routes.lock().await.push(route);
        } else {
            i.inject_type(Mutex::new(vec![route])).await?;
        }

        Ok(())
    }
}
