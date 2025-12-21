mod cli;
mod config;
mod commands;
mod utils;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<()> {
    fmt::init();

    let cli = cli::Cli::parse();
    let config = config::Config::load(cli.config.as_deref())?;

    commands::dispath(cli, config).await
}