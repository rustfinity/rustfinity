use crate::{commands::submit::submit_challenge, config::Config, device_flow, download::get_challenge};
use clap::{Parser, Subcommand};

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Get { command: get } => match get {
            Get::Challenge { challenge } => get_challenge(&challenge).await,
        },
        Commands::Submit => submit_challenge().await,
        Commands::Login => {
            let api_key = device_flow::device_login().await?;
            Config::save(&api_key)?;
            println!();
            println!("Login successful! API key saved.");
            Ok(())
        }
        Commands::Logout => {
            if Config::delete()? {
                println!("Logged out. Config file removed.");
            } else {
                println!("Not logged in (no config file found).");
            }
            Ok(())
        }
        Commands::Whoami => {
            match Config::load() {
                Ok(config) => {
                    let masked = if config.api_key.len() > 7 {
                        format!("{}...", &config.api_key[..7])
                    } else {
                        config.api_key
                    };
                    println!("Logged in with key: {}", masked);
                }
                Err(_) => {
                    println!("Not logged in. Run `rustfinity login` to authenticate.");
                }
            }
            Ok(())
        }
    }
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
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

    /// Authenticate with Rustfinity Cloud
    Login,

    /// Remove saved credentials
    Logout,

    /// Show current authentication status
    Whoami,
}

#[derive(Subcommand)]
enum Get {
    Challenge { challenge: String },
}
