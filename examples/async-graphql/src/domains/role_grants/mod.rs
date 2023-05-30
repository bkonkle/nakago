//! # Role Grants

/// Service
pub mod service;

/// Model
pub mod model;

/// RoleGrant DataLoader
pub mod loader;

pub use loader::{Provider as RoleGrantLoaderProvider, RoleGrantLoader, ROLE_GRANT_LOADER};
pub use service::{
    DefaultService as DefaultRoleGrantsService, Provider as RoleGrantsServiceProvider,
    Service as RoleGrantsService, ROLE_GRANTS_SERVICE,
};

#[cfg(test)]
pub use service::MockService as MockRoleGrantsService;
