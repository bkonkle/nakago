//! The main entry point for the async-graphql example.
#![forbid(unsafe_code)]

use config::AppConfig;
use log::info;
use nakago_axum::HttpApplication;
use providers::{InitApp, StartApp};
use router::AppState;

mod config;
mod db;
mod domains;
mod events;
mod graphql;
mod handlers;
mod providers;
mod router;
mod utils;

/// Error macros
#[macro_use]
extern crate anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app =
        HttpApplication::<AppConfig, AppState>::with_init(router::init(), InitApp::default())
            .and_startup(StartApp::default());

    let server = app.run(None).await?;
    let addr = server.local_addr();

    info!("Started on port: {port}", port = addr.port());

    server.await?;

    Ok(())
}
