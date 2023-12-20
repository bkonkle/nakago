use std::any::Any;

use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};
use tokio::sync::Mutex;
use warp::{filters::BoxedFilter, reply::Reply, wrap_fn, Filter};

/// A Route that will be nested within a higher-level Router, wrapped in a Mutex to safely move
pub type Route = BoxedFilter<(Box<dyn Reply>,)>;

/// A collection of Routes
pub type Routes = Mutex<Vec<Route>>;

/// A hook to initialize a particular route
pub struct Init<F>
where
    F: Fn(BoxedFilter<(Inject,)>) -> BoxedFilter<(Box<dyn Reply>,)> + Clone,
{
    filter: F,
}

impl<F> Init<F>
where
    F: Fn(BoxedFilter<(Inject,)>) -> BoxedFilter<(Box<dyn Reply>,)> + Clone,
{
    /// Create a new Init instance
    pub fn new(filter: F) -> Self {
        Self { filter }
    }
}

#[async_trait]
impl<F> Hook for Init<F>
where
    F: Fn(BoxedFilter<(Inject,)>) -> BoxedFilter<(Box<dyn Reply>,)> + Send + Sync + Any + Clone,
{
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let route = with_injection(i.clone()).with(wrap_fn(self.filter.clone()));

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
    warp::any().map(move || i.clone()).boxed()
}
