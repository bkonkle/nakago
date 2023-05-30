use async_trait::async_trait;
use tracing_subscriber::prelude::*;

use crate::{Hook, InjectResult};

/// An Init Hook to setup RUST_LOG tracing
#[derive(Default)]
pub struct InitRustLog {}

#[async_trait]
impl Hook for InitRustLog {
    async fn handle(&self, _i: &mut crate::Inject) -> InjectResult<()> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        Ok(())
    }
}
