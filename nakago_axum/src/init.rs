use std::{io, net::SocketAddr, panic::PanicInfo, sync::Arc};

use axum::{serve::Serve, Router};
use backtrace::Backtrace;
use crossterm::{execute, style::Print};
use derive_new::new;
use nakago::{Inject, Tag};
use nakago_figment::FromRef;
use tokio::net::TcpListener;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{self, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::Config;

// Server Initialization
// ---------------------

/// TCP Listener Initialization
#[derive(Debug, Clone, Default, new)]
pub struct Listener<C: nakago_figment::Config> {
    config_tag: Option<&'static Tag<C>>,
}

impl<C: nakago_figment::Config> Listener<C> {
    /// Initialize the TCP Listener
    pub async fn init(
        &self,
        i: &Inject,
        router: Router,
    ) -> nakago::Result<(Serve<Router, Router>, SocketAddr)>
    where
        Config: FromRef<C>,
    {
        let config = if let Some(tag) = self.config_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<C>().await?
        };

        let http = Config::from_ref(&*config);

        let addr: SocketAddr = format!("0.0.0.0:{}", http.port)
            .parse()
            .expect("Unable to parse bind address");

        let listener = TcpListener::bind(&addr)
            .await
            .unwrap_or_else(|_| panic!("Unable to bind to address: {}", addr));

        let actual_addr = listener
            .local_addr()
            .map_err(|e| nakago::Error::Any(Arc::new(e.into())))?;

        let server = axum::serve(listener, router);

        Ok((server, actual_addr))
    }
}

// Tracing
// -------

/// Initialize the tracing subscriber
pub fn rust_log_subscriber() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Add the tracing layer to the HTTP server
pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
}

// Panic Handling
// --------------

/// A generic function to log stacktraces on panic
pub fn handle_panic(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        execute!(
            io::stdout(),
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            ))
        )
        .unwrap();
    }
}
