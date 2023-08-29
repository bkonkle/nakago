use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{
    config::{loader::Config, AddConfigLoaders},
    Application, InjectResult,
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
#[derive(Default)]
pub struct AxumApplication<C>
where
    C: Config + Debug,
{
    app: Application<C>,
}

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

impl<C> AxumApplication<C>
where
    C: Config + Debug,
{
    /// Load the App's dependencies and configuration. Triggers the Load lifecycle event.
    pub async fn load(&mut self) -> InjectResult<()> {
        // Add the HTTP Config Initializer
        self.handle(AddConfigLoaders::new(default_http_config_loaders()))
            .await?;

        self.app.load().await
    }

    /// Run the server and return the bound address and a `Future`. Triggers the Startup lifecycle
    /// event.
    ///
    /// **Depends on:**
    ///   - `C: Config`
    ///   - `S: State`
    pub async fn run<S: State>(
        &mut self,
        config_path: Option<PathBuf>,
    ) -> InjectResult<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        HttpConfig: FromRef<C>,
    {
        self.load().await?;
        self.init(config_path).await?;
        self.start().await?;

        let router = self.get_router::<S>().await?;
        let config = self.get_type::<C>().await?;

        let http = HttpConfig::from_ref(&*config);

        let server = Server::bind(
            &format!("0.0.0.0:{}", http.port)
                .parse()
                .expect("Unable to parse bind address"),
        )
        .serve(router.into_make_service());

        Ok(server)
    }

    async fn get_router<S: State>(&self) -> InjectResult<Router> {
        let mut router = Router::<S>::new();

        if let Some(routes) = self.app.get_type_opt::<Mutex<Vec<Route<S>>>>().await? {
            let routes: Vec<Route<S>> = routes.lock().await.drain(..).collect();
            for route in routes {
                router = router.nest(&route.path, route.router.into_inner());
            }
        };

        let state = (*self.app.get_type::<S>().await?).clone();

        let router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(router.with_state(state));

        Ok(router)
    }
}
