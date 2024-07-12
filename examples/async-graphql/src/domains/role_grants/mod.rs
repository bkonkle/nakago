//! # Role Grants

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Schema
pub mod schema;

/// Service
pub mod service;

/// Tests
#[cfg(test)]
mod tests;

pub use loaders::Loader;
pub use service::Service;
