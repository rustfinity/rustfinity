use crate::{commands::{deploy, submit::submit_challenge}, config::Config, constants::api_base_url, device_flow, download::get_challenge};
use clap::{Parser, Subcommand};
use serde::Deserialize;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Get { command: get } => match get {
            Get::Challenge { challenge } => get_challenge(&challenge).await,
        },
        Commands::Submit => submit_challenge().await,
        Commands::Deploy => deploy::deploy().await,
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
                        config.api_key.clone()
                    };
                    println!("Logged in with key: {}", masked);

                    // Verify key against the server
                    let base_url = api_base_url();
                    let url = format!("{}/auth/whoami", base_url);
                    let client = reqwest::Client::new();

                    match client
                        .get(&url)
                        .header("Authorization", format!("Bearer {}", config.api_key))
                        .send()
                        .await
                    {
                        Ok(res) if res.status().is_success() => {
                            #[derive(Deserialize)]
                            struct WhoamiResponse {
                                username: String,
                            }
                            if let Ok(body) = res.json::<WhoamiResponse>().await {
                                println!("Authenticated as: {}", body.username);
                            }
                        }
                        Ok(res) if res.status().as_u16() == 401 => {
                            println!("Warning: API key is not valid on the server. Try `rustfinity login` again.");
                        }
                        Ok(_) | Err(_) => {
                            println!("Warning: Could not reach server to verify key.");
                        }
                    }
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

    /// Deploy the current project to Rustfinity Cloud
    Deploy,

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
