use std::pin::Pin;

use futures::Future;
use tracing_subscriber::prelude::*;

use crate::{Inject, InjectResult};

/// An Init Hook to setup RUST_LOG tracing
pub fn init_rust_log<'a>(
    i: &'a mut Inject,
) -> Pin<Box<dyn Future<Output = InjectResult<()>> + 'a>> {
    Box::pin(async move {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        Ok(())
    })
}
