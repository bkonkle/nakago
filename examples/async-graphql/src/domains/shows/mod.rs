//! # Shows

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Mutations
pub mod mutation;

/// GraphQL Schema
pub mod schema;

/// Service
pub mod service;

/// GraphQL Queries
pub mod query;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loaders::Loader;
pub use mutation::ShowsMutation as Mutation;
pub use query::ShowsQuery as Query;
pub use service::Service;
