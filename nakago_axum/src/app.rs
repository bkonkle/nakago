use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{app::Application, config::loader::Config};
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
    /// Create a new Application instance
    pub fn new(router: Router<S>) -> Self {
        Self {
            app: Application::default(),
            router,
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
    ) -> anyhow::Result<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        HttpConfig: FromRef<C>,
    {
        println!(">------ init_config_loaders ------<");

        // Add the HTTP Config Initializer
        init_config_loaders(&mut self.app).await?;

        println!(">------ self.app.init ------<");

        // Initialize the underlying App
        self.app.start(config_path).await?;

        println!(">------ state ------<");

        let state = self.app.get_type::<S>()?;

        println!(">------ router ------<");

        let app: Router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(self.router.clone().with_state(state.clone()));

        println!(">------ config ------<");

        let config = self.app.get_type::<C>()?;
        let http = HttpConfig::from_ref(config);

        println!(">------ server ------<");

        let server = Server::bind(
            &format!("0.0.0.0:{}", http.port)
                .parse()
                .expect("Unable to parse bind address"),
        )
        .serve(app.into_make_service());

        Ok(server)
    }
}
