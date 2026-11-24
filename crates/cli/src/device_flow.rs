use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::time::Duration;

use crate::constants::api_base_url;

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_url: String,
    interval: u64,
}

#[derive(Deserialize)]
struct TokenResponse {
    api_key: Option<String>,
    error: Option<String>,
}

fn is_web_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("https://") || lower.starts_with("http://")
}

pub async fn device_login() -> Result<String> {
    let client = reqwest::Client::new();
    let base_url = api_base_url();

    // Step 1: Request device code
    let res = client
        .post(format!("{}/auth/device/code", base_url))
        .send()
        .await
        .context("Failed to connect to Rustfinity API")?;

    if !res.status().is_success() {
        bail!("Failed to initiate device login (status {})", res.status());
    }

    let device: DeviceCodeResponse = res.json().await.context("Invalid response from API")?;

    // Step 2: Display code and open browser
    println!();
    println!("  ┌─────────────────────────────────────────┐");
    println!("  │                                         │");
    println!("  │   Your verification code is:            │");
    println!("  │                                         │");
    println!(
        "  │           {:^8}                     │",
        device.user_code
    );
    println!("  │                                         │");
    println!("  │   Opening browser to complete login...  │");
    println!("  │                                         │");
    println!("  └─────────────────────────────────────────┘");
    println!();

    // Only hand http(s) URLs to the OS opener; anything else (file://, custom
    // schemes, ...) from the server would be opened by an arbitrary handler.
    if !is_web_url(&device.verification_url) {
        bail!("Unexpected verification URL from API");
    }

    if webbrowser::open(&device.verification_url).is_err() {
        println!("Could not open browser. Please visit:");
        println!("  {}", device.verification_url);
        println!();
    }

    println!("Waiting for authorization...");

    // Step 3: Poll for token
    // Never poll faster than once a second, even if the server says 0.
    let interval = Duration::from_secs(device.interval.max(1));

    loop {
        tokio::time::sleep(interval).await;

        let res = client
            .post(format!("{}/auth/device/token", base_url))
            .json(&serde_json::json!({ "device_code": device.device_code }))
            .send()
            .await
            .context("Failed to poll for token")?;

        let token_res: TokenResponse = res.json().await.context("Invalid token response")?;

        if let Some(api_key) = token_res.api_key {
            return Ok(api_key);
        }

        match token_res.error.as_deref() {
            Some("authorization_pending") => continue,
            Some("expired_token") => bail!("Device code expired. Please try again."),
            Some("access_denied") => bail!("Authorization was denied."),
            Some(err) => bail!("Unexpected error: {}", err),
            None => bail!("Unexpected response from API"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_web_urls_are_opened() {
        assert!(is_web_url("https://www.rustfinity.com/device"));
        assert!(is_web_url("HTTP://localhost:3000/device"));
        assert!(!is_web_url("file:///etc/passwd"));
        assert!(!is_web_url("javascript:alert(1)"));
        assert!(!is_web_url("smb://evil/share"));
    }
}
