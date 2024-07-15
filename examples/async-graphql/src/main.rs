//! The main entry point for the async-graphql example.
#![forbid(unsafe_code)]

use std::path::PathBuf;

use log::info;
use pico_args::{Arguments, Error};

mod authz;
mod config;
mod domains;
mod http;
mod init;
mod messages;

pub use config::Config;

/// Error macros
#[macro_use]
extern crate anyhow;

const HELP: &str = "\
Usage: async-graphql [OPTIONS]

Options:
  -c, --config <FILE>  Sets a custom config file path
  -h, --help           Print help
";

#[derive(Debug)]
struct Args {
    /// Prints the usage menu
    help: bool,

    /// Sets a custom config file path
    config_path: Option<PathBuf>,
}

impl Args {
    pub fn parse() -> Result<Args, Error> {
        let mut pargs = Arguments::from_env();

        let args = Args {
            help: pargs.contains(["-h", "--help"]),
            config_path: pargs.opt_value_from_str(["-c", "--config"])?,
        };

        Ok(args)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;

    if args.help {
        println!("{}", HELP);
        return Ok(());
    }

    let app = init::app().await?;
    let (server, addr) = app.run(args.config_path).await?;

    info!("Started on port: {port}", port = addr.port());

    server.await?;

    Ok(())
}
