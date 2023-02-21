use async_trait::async_trait;
use axum::Router;
use std::any::Any;

/// The AppRouter trait, which initializes a top-level Router for Axum
#[async_trait]
pub trait AppRouter: Any + Send + Sync {
    /// Initialize the AppRouter, returning a top-level Router for Axum
    async fn init(&self) -> Router;
}
