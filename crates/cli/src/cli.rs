use crate::{commands::submit::submit_challenge, download::get_challenge};
use clap::{Parser, Subcommand};

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Get { command: get } => match get {
            Get::Challenge { challenge } => get_challenge(&challenge).await,
        },
        Commands::Submit => submit_challenge().await,
    }
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

    /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        #[clap(subcommand)]
        command: Get,
    },

    Submit,
}

#[derive(Subcommand)]
enum Get {
    Challenge { challenge: String },
}
