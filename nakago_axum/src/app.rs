use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{config::loader::Config, inject, Application};
use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    path::PathBuf,
};
use tower_http::trace;

use crate::{add_http_config_loaders, config::HttpConfig};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// An Axum HTTP Application
#[derive(Default)]
pub struct AxumApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    sys: Application<C>,
    router: Router<S>,
}

impl<C, S> Deref for AxumApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.sys
    }
}

impl<C, S> DerefMut for AxumApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sys
    }
}

impl<C, S> AxumApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    /// Create a new Application instance with a startup and shutdown hook
    pub fn with_hooks<H1: inject::Hook, H2: inject::Hook>(
        router: Router<S>,
        init: H1,
        startup: H2,
    ) -> Self {
        Self {
            sys: Application::with_hooks(init, startup),
            router,
        }
    }

    /// Create a new Application instance with an init hook
    pub fn with_init<H: inject::Hook>(router: Router<S>, init: H) -> Self {
        Self {
            sys: Application::with_init(init),
            router,
        }
    }

    /// Create a new Application instance with a startup hook
    pub fn with_startup<H: inject::Hook>(router: Router<S>, startup: H) -> Self {
        Self {
            sys: Application::with_startup(startup),
            router,
        }
    }

    /// Set the init hook
    pub fn and_init<H: inject::Hook>(self, init: H) -> Self {
        Self {
            sys: self.sys.and_init(init),
            ..self
        }
    }

    /// Set the startup hook
    pub fn and_startup<H: inject::Hook>(self, startup: H) -> Self {
        Self {
            sys: self.sys.and_startup(startup),
            ..self
        }
    }

    /// Start the server and return the bound address and a `Future`.
    ///
    /// **Depends on:**
    ///   - `C: Config`
    ///   - `S: State`
    pub async fn run(
        &mut self,
        config_path: Option<PathBuf>,
    ) -> inject::Result<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        HttpConfig: FromRef<C>,
    {
        self.init(config_path).await?;

        // Run the startup hook
        self.start().await?;

        let state = self.sys.get_type::<S>()?;

        let app: Router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(self.router.clone().with_state(state.clone()));

        let config = self.sys.get_type::<C>()?;
        let http = HttpConfig::from_ref(config);

        let server = Server::bind(
            &format!("0.0.0.0:{}", http.port)
                .parse()
                .expect("Unable to parse bind address"),
        )
        .serve(app.into_make_service());

        Ok(server)
    }

    /// Initialize the underlying App
    pub async fn init(&mut self, config_path: Option<PathBuf>) -> inject::Result<()> {
        // Add the HTTP Config Initializer
        self.sys.handle(add_http_config_loaders()).await?;

        self.sys.init(config_path).await
    }

    /// Start up the underlying App
    pub async fn start(&mut self) -> inject::Result<()> {
        self.sys.start().await
    }
}
