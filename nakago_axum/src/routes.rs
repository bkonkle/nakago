use async_trait::async_trait;
use axum::Router;
use nakago::{hooks, Hook, Inject};
use tokio::sync::Mutex;

use crate::State;

/// A Route that will be nested within a higher-level Router, wrapped in a Mutex to safely move
pub type Route = Mutex<Router<State>>;

/// A collection of Routes
pub type Routes = Mutex<Vec<Route>>;

/// A hook to initialize a particular route
pub struct Init {
    router: Mutex<Router<State>>,
}

impl Init {
    /// Create a new Init instance
    pub fn new(router: Router<State>) -> Self {
        Self {
            router: Mutex::new(router),
        }
    }
}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let router = self.router.lock().await.clone();

        if let Some(routes) = i.get_opt::<Routes>().await? {
            routes.lock().await.push(Mutex::new(router));
        } else {
            i.inject::<Routes>(Mutex::new(vec![Mutex::new(router)]))
                .await?;
        }

        Ok(())
    }
}
