//! The main entry point for the simple-warp example.
#![forbid(unsafe_code)]

use std::path::PathBuf;

use config::Config;
use http::router;
use log::info;
use nakago_warp::init::Listener;
use pico_args::{Arguments, Error};

mod config;
mod http;
mod init;

const HELP: &str = "\
Usage: simple-warp [OPTIONS]

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

    let i = init::app(args.config_path).await?;

    let router = router::init(&i);

    let (server, addr) = Listener::<Config>::default().init(&i, router).await?;

    info!("Started on port: {port}", port = addr.port());

    server.await;

    Ok(())
}