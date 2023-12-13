use std::{
    fmt::Debug,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
};

use anyhow::anyhow;
use nakago::{hooks, utils::FromRef, Application, Tag};
use tokio::sync::Mutex;
use warp::{filters::BoxedFilter, reject::Rejection, reply::Reply, Filter, Future};

use crate::{config::Config, errors::handle_rejection};

/// A Warp HTTP Application
pub struct WarpApplication<C>
where
    C: nakago::Config,
{
    app: Application<C>,
}

impl<C> WarpApplication<C>
where
    C: nakago::Config,
{
    /// Create a new WarpApplication instance
    pub fn new(config_tag: Option<&'static Tag<C>>) -> Self {
        Self {
            app: Application::new(config_tag),
        }
    }

    /// Add a config tag for the Application to use
    pub fn with_config_tag(self, tag: &'static Tag<C>) -> Self {
        Self {
            app: self.app.with_config_tag(tag),
        }
    }
}

impl<C> Default for WarpApplication<C>
where
    C: nakago::Config,
{
    fn default() -> Self {
        Self::new(None)
    }
}

impl<C> Deref for WarpApplication<C>
where
    C: nakago::Config + Debug,
{
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<C> DerefMut for WarpApplication<C>
where
    C: nakago::Config + Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<C> WarpApplication<C>
where
    C: nakago::Config + Debug,
{
    /// Run the server and return the bound address and a `Future`. Triggers the Startup lifecycle
    /// event.
    ///
    /// **Depends on:**
    ///   - `C: Config`
    ///   - `S: State`
    pub async fn run(
        &self,
        config_path: Option<PathBuf>,
    ) -> hooks::Result<(impl Future<Output = ()>, SocketAddr)>
    where
        Config: FromRef<C>,
    {
        self.load(config_path).await?;
        self.init().await?;
        self.start().await?;

        let router = self.get_router().await?;
        let config = self.get_config().await?;

        let http = Config::from_ref(&*config);

        let addr: SocketAddr = format!("0.0.0.0:{}", http.port)
            .parse()
            .expect("Unable to parse bind address");

        let (actual_addr, server) =
            warp::serve(router.with(warp::log("warp")).recover(handle_rejection))
                .bind_ephemeral(addr);

        Ok((server, actual_addr))
    }

    async fn get_router(&self) -> hooks::Result<BoxedFilter<(Box<dyn Reply>,)>> {
        if let Some(routes) = self.app.get_type_opt::<Routes>().await? {
            if routes.length() > 0 {
                if let Some(route) = routes.lock().await.drain(..).reduce(|a, b| a.or(b).boxed()) {
                    return Ok(route);
                };
            }
        }

        Err(hooks::Error::Any(Arc::new(anyhow!(
            "No routes defined for application"
        ))))?
    }
}

pub type Route = BoxedFilter<(Box<dyn Reply>,)>;

pub type Routes = Mutex<Vec<Route>>;
