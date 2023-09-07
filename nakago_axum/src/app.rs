use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{
    config::{loader::Config, AddConfigLoaders},
    Application, InjectResult, Tag,
};
use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    path::PathBuf,
};
use tokio::sync::Mutex;
use tower_http::trace;

use crate::{
    config::{default_http_config_loaders, HttpConfig},
    Route,
};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// An Axum HTTP Application
pub struct AxumApplication<C, S>
where
    C: Config,
    S: State,
{
    app: Application<C>,
    config_tag: Option<&'static Tag<C>>,
    state_tag: Option<&'static Tag<S>>,
}

impl<C, S> AxumApplication<C, S>
where
    C: Config,
    S: State,
{
    /// Create a new AxumApplication instance
    pub fn new(config_tag: Option<&'static Tag<C>>, state_tag: Option<&'static Tag<S>>) -> Self {
        Self {
            app: Application::new(config_tag),
            config_tag,
            state_tag,
        }
    }

    /// Add a config tag for the Application to use
    pub fn with_config_tag(self, tag: &'static Tag<C>) -> Self {
        Self {
            config_tag: Some(tag),
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
    C: Config,
    S: State,
{
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl<C, S> Deref for AxumApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<C, S> DerefMut for AxumApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<C, S> AxumApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    /// Load the App's dependencies and configuration. Triggers the Load lifecycle event.
    pub async fn load(&mut self, config_path: Option<PathBuf>) -> InjectResult<()> {
        // Add the default HTTP Config loaders
        self.handle(AddConfigLoaders::new(default_http_config_loaders()))
            .await?;

        self.app.load(config_path).await
    }

    /// Run the server and return the bound address and a `Future`. Triggers the Startup lifecycle
    /// event.
    ///
    /// **Depends on:**
    ///   - `C: Config`
    ///   - `S: State`
    pub async fn run(
        &mut self,
        config_path: Option<PathBuf>,
    ) -> InjectResult<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        HttpConfig: FromRef<C>,
    {
        self.load(config_path).await?;
        self.init().await?;
        self.start().await?;

        let router = self.get_router().await?;

        let config = if let Some(tag) = self.config_tag {
            self.app.get(tag).await?
        } else {
            self.app.get_type::<C>().await?
        };

        let http = HttpConfig::from_ref(&*config);

        let server = Server::bind(
            &format!("0.0.0.0:{}", http.port)
                .parse()
                .expect("Unable to parse bind address"),
        )
        .serve(router.into_make_service());

        Ok(server)
    }

    async fn get_router(&self) -> InjectResult<Router> {
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
