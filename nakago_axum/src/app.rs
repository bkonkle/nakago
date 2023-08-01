use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{config::loader::Config, inject, Application};
use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    path::PathBuf,
};
use tokio::sync::Mutex;
use tower_http::trace;

use crate::{add_http_config_loaders, config::HttpConfig, Route};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

impl<C> Deref for AxumApplication<C>
where
    C: Config + Debug,
{
    type Target = Application<C>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<C> DerefMut for AxumApplication<C>
where
    C: Config + Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

/// An Axum HTTP Application
#[derive(Default)]
pub struct AxumApplication<C>
where
    C: Config + Debug,
{
    app: Application<C>,
}

impl<C> AxumApplication<C>
where
    C: Config + Debug,
{
    /// Initialize the underlying App
    pub async fn init(&mut self, config_path: Option<PathBuf>) -> inject::Result<()> {
        // Add the HTTP Config Initializer
        self.app.handle(add_http_config_loaders()).await?;

        self.app.init(config_path).await
    }

    /// Start the server and return the bound address and a `Future`.
    ///
    /// **Depends on:**
    ///   - `C: Config`
    ///   - `S: State`
    pub async fn run<S: State>(
        &mut self,
        config_path: Option<PathBuf>,
    ) -> inject::Result<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        HttpConfig: FromRef<C>,
    {
        // Trigger the Init lifecycle event
        self.init(config_path).await?;

        // Trigger the Startup lifecycle event
        self.start().await?;

        let mut router = Router::<S>::new();

        if let Some(routes) = self.app.get_type_opt::<Mutex<Vec<Route<S>>>>()? {
            let routes: Vec<Route<S>> = routes.lock().await.drain(..).collect();
            for route in routes {
                router = router.nest(&route.path, route.router.into_inner());
            }
        };

        let state = self.app.get_type::<S>()?.clone();

        let app: Router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(router.with_state(state));

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
}
