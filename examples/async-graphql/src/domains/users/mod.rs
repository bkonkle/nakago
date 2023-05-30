//! # Users

/// Service
pub mod service;

/// Model
pub mod model;

/// GraphQL Mutations
pub mod mutations;

/// GraphQL Resolver
pub mod resolver;

/// User DataLoader
pub mod loader;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loader::{Provider as UserLoaderProvider, UserLoader, USER_LOADER};
pub use service::{
    DefaultService as DefaultUsersService, Provider as UsersServiceProvider,
    Service as UsersService, USERS_SERVICE,
};

#[cfg(test)]
pub use service::MockService as MockUsersService;
