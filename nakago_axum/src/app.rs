use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{self, inject, Application, Tag};
use tokio::sync::Mutex;
use tower_http::trace;

use crate::{Config, Route};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// An Axum HTTP Application
pub struct AxumApplication<C, S>
where
    C: nakago::Config,
    S: State,
{
    app: Application<C>,
    state_tag: Option<&'static Tag<S>>,
}

impl<C, S> AxumApplication<C, S>
where
    C: nakago::Config,
    S: State,
{
    /// Create a new AxumApplication instance
    pub fn new(config_tag: Option<&'static Tag<C>>, state_tag: Option<&'static Tag<S>>) -> Self {
        Self {
            app: Application::new(config_tag),
            state_tag,
        }
    }

    /// Add a config tag for the Application to use
    pub fn with_config_tag(self, tag: &'static Tag<C>) -> Self {
        Self {
            app: self.app.with_config_tag(tag),
            ..self
        }
    }

    /// Add a state tag for the Application to use
    pub fn with_state_tag(self, tag: &'static Tag<S>) -> Self {
        Self {
            state_tag: Some(tag),
            ..self
        }
    }
}

impl<C, S> Default for AxumApplication<C, S>
where
    C: nakago::Config,
    S: State,
{
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl<C, S> Deref for AxumApplication<C, S>
where
    C: nakago::Config + Debug,
    S: State,
{
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<C, S> DerefMut for AxumApplication<C, S>
where
    C: nakago::Config + Debug,
    S: State,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<C, S> AxumApplication<C, S>
where
    C: nakago::Config + Debug,
    S: State,
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
    ) -> inject::Result<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        Config: FromRef<C>,
    {
        self.load(config_path).await?;
        self.init().await?;
        self.start().await?;

        let router = self.get_router().await?;
        let config = self.get_config().await?;

        let http = Config::from_ref(&*config);

        let server = Server::bind(
            &format!("0.0.0.0:{}", http.port)
                .parse()
                .expect("Unable to parse bind address"),
        )
        .serve(router.into_make_service());

        Ok(server)
    }

    async fn get_router(&self) -> inject::Result<Router> {
        let mut router = Router::<S>::new();

        if let Some(routes) = self.app.get_type_opt::<Mutex<Vec<Route<S>>>>().await? {
            let routes: Vec<Route<S>> = routes.lock().await.drain(..).collect();
            for route in routes {
                router = router.nest(&route.path, route.router.into_inner());
            }
        };

        let state = if let Some(tag) = self.state_tag {
            self.app.get(tag).await?
        } else {
            self.app.get_type::<S>().await?
        };

        let router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(router.with_state((*state).clone()));

        Ok(router)
    }
}
