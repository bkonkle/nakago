use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};
use tokio::sync::Mutex;
use warp::{filters::BoxedFilter, reply::Reply};

/// A Route that will be nested within a higher-level Router, wrapped in a Mutex to safely move
pub type Route = BoxedFilter<(Box<dyn Reply>,)>;

/// A collection of Routes
pub type Routes = Mutex<Vec<Route>>;

/// A hook to initialize a particular route
pub struct Init {
    route: Route,
}

impl Init {
    /// Create a new Init instance
    pub fn new(route: Route) -> Self {
        Self { route }
    }
}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        if let Some(routes) = i.get_type_opt::<Routes>().await? {
            routes.lock().await.push(self.route.clone());
        } else {
            i.inject_type::<Routes>(Mutex::new(vec![self.route.clone()]))
                .await?;
        }

        Ok(())
    }
}
