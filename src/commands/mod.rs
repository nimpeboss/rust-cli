use anyhow::Result;

use crate::{cli::{self, Cli}, config::{self, Config}};

pub mod scan;
pub mod hash;

pub async fn dispath(cli: Cli, config: Config) -> Result<()> {
    match cli.command {
        crate::cli::Commands::Scan { path } => {
            scan::run(path).await
        }
        crate::cli::Commands::Hash { file } => {
            hash::run(file).await
        }
    }
}