mod auth;
mod cargo_toml;
mod challenge;
mod cli;
mod commands;
mod config;
mod confirm;
mod constants;
mod crates_io;
mod device_flow;
mod dir;
mod download;

use anyhow::Context;
use clap::Parser;
use cli::{run, Cli};
use semver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let latest_version = crates_io::get_latest_version().await?;
    let latest_version = semver::Version::parse(&latest_version)?;

    let current_version = env!("CARGO_PKG_VERSION");
    let current_version = semver::Version::parse(current_version)?;

    if latest_version > current_version {
        let yes = confirm::confirm(
            &format!(
                "A new version of rustfinity ({}) is available. Would you like to update now?",
                latest_version
            ),
            true,
        )
        .context("Failed to read input")?;

        if yes {
            let status = std::process::Command::new("cargo")
                .args(["install", "rustfinity"])
                .status()
                .context("Failed to run cargo install")?;

            if !status.success() {
                anyhow::bail!("cargo install rustfinity failed");
            }

            println!("Updated successfully! Please re-run your command.");
        }

        return Ok(());
    }

    let cli = Cli::parse();
    run(cli).await
}
