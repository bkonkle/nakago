use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{app::Application, config::loader::Config, inject};
use std::{any::Any, ops::Deref};
use tower_http::trace;

use crate::config::HttpConfig;

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// The top-level Application struct
pub struct HttpApplication<C: Config, S: State> {
    app: Application<C>,
    router: Router<S>,
}

impl<C: Config, S: State> Deref for HttpApplication<C, S> {
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<C: Config, S: State> HttpApplication<C, S> {
    /// Create a new Application instance
    pub fn new(initializers: Vec<Box<dyn inject::Initializer>>, router: Router<S>) -> Self {
        Self {
            app: Application::new(initializers),
            router,
        }
    }

    /// Start the server and return the bound address and a `Future`.
    ///
    /// **Depends on:**
    ///   - `C: Config`
    ///   - `S: State`
    pub async fn run(&mut self) -> anyhow::Result<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        HttpConfig: FromRef<C>,
    {
        let state = self.app.get::<S>()?;

        let app: Router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(self.router.clone().with_state(state.clone()));

        let config = self.app.get::<C>()?;
        let http = HttpConfig::from_ref(config);

        let server = Server::bind(
            &format!("0.0.0.0:{}", http.port)
                .parse()
                .expect("Unable to parse bind address"),
        )
        .serve(app.into_make_service());

        Ok(server)
    }
}
