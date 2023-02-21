use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{
    config::{loader::Config, providers::ConfigInitializer},
    inject,
};
use std::{any::Any, marker::PhantomData, ops::Deref, path::PathBuf};
use tower_http::trace;

use crate::config::HttpConfig;

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// The top-level Application struct
pub struct Application<C: Config, S: State> {
    initializers: Vec<Box<dyn inject::Initializer>>,
    router: Router<S>,
    i: inject::Inject,
    _phantom: PhantomData<(C, S)>,
}

impl<C: Config, S: State> Deref for Application<C, S> {
    type Target = inject::Inject;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

impl<C: Config, S: State> Application<C, S> {
    /// Create a new Application instance
    pub fn new(initializers: Vec<Box<dyn inject::Initializer>>, router: Router<S>) -> Self {
        Self {
            initializers,
            router,
            i: inject::Inject::default(),
            _phantom: Default::default(),
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
        // 先ず First of all, initialize the Config
        self.i
            .init(if let Some(config_path) = config_path {
                vec![Box::new(ConfigInitializer::<C>::with_custom_path(
                    config_path,
                ))]
            } else {
                vec![Box::new(ConfigInitializer::<C>::default())]
            })
            .await?;

        for initializer in &self.initializers {
            initializer.init(&mut self.i).await?;
        }

        let state = self.i.get::<S>()?;

        let app: Router = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(self.router.clone().with_state(state.clone()));

        let config = self.i.get::<C>()?;
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
