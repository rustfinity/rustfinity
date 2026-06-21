use anyhow::Result;

use crate::{config::Config, device_flow};

/// Ensures user is authenticated, automatically triggering login if needed.
/// Returns a valid Config instance.
///
/// Uses lazy verification: doesn't validate API key with server upfront.
/// Invalid keys will be detected on actual API calls (401 response).
pub async fn ensure_authenticated() -> Result<Config> {
    match Config::load() {
        Ok(config) => Ok(config),
        Err(_) => {
            // No valid config found - trigger login flow
            perform_login().await
        }
    }
}

/// Performs device flow login and saves credentials.
/// Returns a valid Config instance.
pub async fn perform_login() -> Result<Config> {
    let api_key = device_flow::device_login().await?;
    Config::save(&api_key)?;
    println!();
    println!("Login successful! API key saved.");

    // Return the config instance
    Ok(Config { api_key })
}
