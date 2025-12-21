use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Scan {
        path: String,
    },

    Hash {
        file: String,
    },
}