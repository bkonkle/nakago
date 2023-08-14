//! # Domains
#![forbid(unsafe_code)]

/// Users
pub mod users;

/// Profiles
pub mod profiles;

/// Role Grants
pub mod role_grants;

/// Shows
pub mod shows;

/// Episodes
pub mod episodes;

/// Dependency injection providers
pub mod init;

pub use init::InitDomains;
