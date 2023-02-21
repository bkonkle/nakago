use axum::{extract::FromRef, routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use nakago::{
    config::{loader::ConfigData, providers::ConfigInitializer},
    inject::Initializer,
    Inject,
};
use std::path::PathBuf;
use tower_http::trace;

use crate::{config::HttpConfig, router::AppRouter};

/// The top-level Application struct
pub struct Application {
    router: Box<dyn AppRouter>,
    init: Box<dyn Initializer>,
}

impl Application {
    /// Create a new Application instance
    pub const fn new(router: Box<dyn AppRouter>, init: Box<dyn Initializer>) -> Self {
        Self { router, init }
    }

    /// Start the server and return the bound address and a `Future`.
    pub async fn run<C: ConfigData>(
        &self,
        custom_path: Option<PathBuf>,
    ) -> anyhow::Result<Server<AddrIncoming, IntoMakeService<Router>>>
    where
        HttpConfig: FromRef<C>,
    {
        let app = Router::new()
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .merge(self.router.init().await);

        let mut i = Inject::default();

        // 先ず First of all, initialize the Config
        i.init(if let Some(custom_path) = custom_path {
            vec![ConfigInitializer::<C>::with_custom_path(custom_path)]
        } else {
            vec![ConfigInitializer::<C>::default()]
        })
        .await?;

        // Then, run the main Initializer
        self.init.init(&mut i).await?;

        let config = i.get::<C>()?;
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
