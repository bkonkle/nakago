use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{
    app::{Application, LifecycleHook},
    config::loader::Config,
    inject,
};
use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    path::PathBuf,
};
use tower_http::trace;

use crate::{config::HttpConfig, init_config_loaders};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// The top-level Application struct
#[derive(Default)]
pub struct HttpApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    app: Application<C>,
    router: Router<S>,
}

impl<C, S> Deref for HttpApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<C, S> DerefMut for HttpApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<C, S> HttpApplication<C, S>
where
    C: Config + Debug,
    S: State,
{
    /// Create a new Application instance with a startup and shutdown hook
    pub fn with_hooks<
        H1: LifecycleHook + Send + 'static,
        H2: LifecycleHook + Send + 'static,
        H3: LifecycleHook + Send + 'static,
    >(
        router: Router<S>,
        init: H1,
        startup: H2,
        shutdown: H3,
    ) -> Self {
        Self {
            app: Application::with_hooks(init, startup, shutdown),
            router,
        }
    }

    /// Create a new Application instance with an init hook
    pub fn with_init<H: LifecycleHook + Send + 'static>(router: Router<S>, init: H) -> Self {
        Self {
            app: Application::with_init(init),
            router,
        }
    }

    /// Create a new Application instance with a startup hook
    pub fn with_startup<H: LifecycleHook + Send + 'static>(router: Router<S>, startup: H) -> Self {
        Self {
            app: Application::with_startup(startup),
            router,
        }
    }

    /// Create a new Application instance with a shutdown hook
    pub fn with_shutdown<H: LifecycleHook + Send + 'static>(
        router: Router<S>,
        shutdown: H,
    ) -> Self {
        Self {
            app: Application::with_shutdown(shutdown),
            router,
        }
    }

    /// Set the init hook
    pub fn and_init<H: LifecycleHook + Send + 'static>(self, init: H) -> Self {
        Self {
            app: self.app.and_init(init),
            ..self
        }
    }

    /// Set the startup hook
    pub fn and_startup<H: LifecycleHook + Send + 'static>(self, startup: H) -> Self {
        Self {
            app: self.app.and_startup(startup),
            ..self
        }
    }

    /// Set the shutdown hook
    pub fn and_shutdown<H: LifecycleHook + Send + 'static>(self, shutdown: H) -> Self {
        Self {
            app: self.app.and_shutdown(shutdown),
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

        let state = self.app.get_type::<S>()?;

        let app: Router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(self.router.clone().with_state(state.clone()));

        let config = self.app.get_type::<C>()?;
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
        init_config_loaders(&mut self.app).await?;

        self.app.init(config_path).await
    }

    /// Start up the underlying App
    pub async fn start(&mut self) -> inject::Result<()> {
        self.app.start().await
    }

    /// Shut down the underlying App
    pub async fn stop(&mut self) -> inject::Result<()> {
        self.app.stop().await
    }
}
