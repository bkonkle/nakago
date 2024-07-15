//! # Users

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Mutation
pub mod mutation;

/// GraphQL Query
pub mod query;

/// GraphQL Schema
pub mod schema;

/// Service
pub mod service;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loaders::Loader;
pub use mutation::UsersMutation as Mutation;
pub use query::UsersQuery as Query;
pub use service::Service;
