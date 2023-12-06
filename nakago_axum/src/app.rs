use std::{
    fmt::Debug,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use axum::{extract::FromRef, serve::Serve, Router};
use nakago::{self, inject, Application, Tag};
use tokio::net::TcpListener;
use tower_http::trace;

use crate::{
    routes::{Route, Routes},
    Config, State,
};

/// An Axum HTTP Application
pub struct AxumApplication<C>
where
    C: nakago::Config,
{
    app: Application<C>,
}

impl<C> AxumApplication<C>
where
    C: nakago::Config,
{
    /// Create a new AxumApplication instance
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

impl<C> Default for AxumApplication<C>
where
    C: nakago::Config,
{
    fn default() -> Self {
        Self::new(None)
    }
}

impl<C> Deref for AxumApplication<C>
where
    C: nakago::Config + Debug,
{
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<C> DerefMut for AxumApplication<C>
where
    C: nakago::Config + Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<C> AxumApplication<C>
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
    ) -> inject::Result<(Serve<Router, Router>, SocketAddr)>
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

        let listener = TcpListener::bind(&addr)
            .await
            .unwrap_or_else(|_| panic!("Unable to bind to address: {}", addr));

        let server = axum::serve(listener, router);

        Ok((server, addr))
    }

    async fn get_router(&self) -> inject::Result<Router> {
        let mut router = Router::<State>::new();

        if let Some(routes) = self.app.get_type_opt::<Routes>().await? {
            let routes: Vec<Route> = routes.lock().await.drain(..).collect();
            for route in routes {
                router = router.nest("/", route.into_inner());
            }
        };

        let router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(router.with_state(State::new(self.app.clone())));

        Ok(router)
    }
}
