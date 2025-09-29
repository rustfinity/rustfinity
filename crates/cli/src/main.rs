mod cargo_toml;
mod challenge;
mod cli;
mod commands;
mod constants;
mod crates_io;
mod dir;
mod download;

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
        println!(
            "A new version of rustfinity ({}) is available, please run the following command and try again:",
            latest_version
        );
        println!("$ cargo install rustfinity");
        return Ok(());
    }

    let cli = Cli::parse();
    run(cli).await
}
