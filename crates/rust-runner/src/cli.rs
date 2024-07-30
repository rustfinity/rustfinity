use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about = "CLI for rust code runner", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        #[clap(short, long)]
        /// Code base64 encoded
        code: String,
    },
}
