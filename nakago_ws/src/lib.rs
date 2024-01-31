//! # nakago-ws: A Warp HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// `WebSockets` Connections
pub mod connections;

/// Event handler
pub mod handler;

/// The Router trait
pub mod router;

pub use connections::Connections;
pub use handler::Handler;
pub use router::{Router, ROUTER};
