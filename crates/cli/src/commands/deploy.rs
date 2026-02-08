use anyhow::{bail, Context, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::config::Config;
use crate::constants::api_base_url;

#[derive(Deserialize)]
struct DeployResponse {
    project_id: String,
    deployment_id: String,
    image: String,
    status: String,
    is_new_project: bool,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    project_id: String,
    project_slug: String,
}

#[derive(Deserialize)]
struct CargoTomlPackage {
    name: String,
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
}

pub async fn deploy() -> Result<()> {
    // 1. Load config (check auth)
    let config = Config::load().context(
        "Not logged in. Run `rustfinity login` to authenticate before deploying.",
    )?;

    // 2. Verify Cargo.toml exists
    let cargo_toml_path = Path::new("Cargo.toml");
    if !cargo_toml_path.exists() {
        bail!("No Cargo.toml found in the current directory. Please run this command from a Rust project root.");
    }

    // 3. Parse Cargo.toml to get package name and derive slug
    let cargo_toml_contents = fs::read_to_string(cargo_toml_path)
        .context("Failed to read Cargo.toml")?;
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_contents)
        .context("Failed to parse Cargo.toml")?;
    let package_name = &cargo_toml.package.name;
    let slug = package_name.replace('_', "-");

    println!("Deploying project: {} (slug: {})", package_name, slug);

    // 4. Load .rustfinity.json if it exists (get project_id)
    let project_config_path = Path::new(".rustfinity.json");
    let existing_project_id = if project_config_path.exists() {
        let contents = fs::read_to_string(project_config_path)
            .context("Failed to read .rustfinity.json")?;
        let project_config: ProjectConfig = serde_json::from_str(&contents)
            .context("Failed to parse .rustfinity.json")?;
        Some(project_config.project_id)
    } else {
        None
    };

    // 5. Build release binary for x86_64-unknown-linux-gnu
    println!("Building release binary...");
    let build_status = Command::new("cargo")
        .args(["build", "--release", "--target", "x86_64-unknown-linux-gnu"])
        .status()
        .context("Failed to run cargo build")?;

    if !build_status.success() {
        bail!("cargo build failed with exit code: {}", build_status);
    }

    // 6. Locate binary
    let binary_path = format!(
        "target/x86_64-unknown-linux-gnu/release/{}",
        package_name
    );
    let binary_path = Path::new(&binary_path);
    if !binary_path.exists() {
        bail!(
            "Expected binary not found at {}. Make sure the package produces a binary target.",
            binary_path.display()
        );
    }

    // 7. Copy binary with unique name
    let short_uuid = &uuid::Uuid::new_v4().to_string()[..8];
    let unique_binary_name = format!("rustfinity-app-{}", short_uuid);
    let temp_binary_path = format!("target/release/{}", unique_binary_name);
    fs::copy(binary_path, &temp_binary_path)
        .context("Failed to copy binary to temp location")?;

    // 8. Create source zip via git archive
    println!("Creating source archive...");
    let source_zip_path = "target/release/rustfinity-source.zip";
    let archive_status = Command::new("git")
        .args([
            "archive",
            "--format=zip",
            &format!("--output={}", source_zip_path),
            "HEAD",
        ])
        .status();

    let has_source_zip = match archive_status {
        Ok(status) if status.success() => true,
        _ => {
            println!("Warning: Could not create source archive (not a git repo?). Continuing without source zip.");
            false
        }
    };

    // 9. Create multipart form and send request
    println!("Uploading to Rustfinity Cloud...");
    let binary_bytes = fs::read(&temp_binary_path)
        .context("Failed to read binary file")?;

    let binary_part = multipart::Part::bytes(binary_bytes)
        .file_name(unique_binary_name.clone())
        .mime_str("application/octet-stream")?;

    let mut form = multipart::Form::new()
        .part("binary", binary_part)
        .text("project_name", package_name.clone())
        .text("project_slug", slug.clone());

    if let Some(ref project_id) = existing_project_id {
        form = form.text("project_id", project_id.clone());
    }

    if has_source_zip {
        let source_bytes = fs::read(source_zip_path)
            .context("Failed to read source zip")?;
        let source_part = multipart::Part::bytes(source_bytes)
            .file_name("rustfinity-source.zip")
            .mime_str("application/zip")?;
        form = form.part("source_zip", source_part);
    }

    // 10. POST to deploy endpoint
    let base_url = api_base_url();
    let url = format!("{}/deploy", base_url);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .multipart(form)
        .send()
        .await
        .context("Failed to send deploy request")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if status.as_u16() == 401 {
            bail!(
                "Deploy failed: Authentication failed. Your API key may be invalid or expired.\n\
                 Run `rustfinity whoami` to check your key, or `rustfinity login` to re-authenticate."
            );
        }
        bail!("Deploy failed (HTTP {}): {}", status, body);
    }

    let deploy_response: DeployResponse = response
        .json()
        .await
        .context("Failed to parse deploy response")?;

    // 11. Save project config to .rustfinity.json
    let project_config = ProjectConfig {
        project_id: deploy_response.project_id.clone(),
        project_slug: slug.clone(),
    };
    let config_json = serde_json::to_string_pretty(&project_config)?;
    fs::write(project_config_path, config_json)
        .context("Failed to write .rustfinity.json")?;

    // 12. Print deployment result
    if deploy_response.is_new_project {
        println!("Created new project: {}", slug);
    } else {
        println!("Redeployed project: {}", slug);
    }
    println!("Deployment ID: {}", deploy_response.deployment_id);
    println!("Image: {}", deploy_response.image);
    println!("Status: {}", deploy_response.status);

    // 13. Cleanup temp files
    let _ = fs::remove_file(&temp_binary_path);
    if has_source_zip {
        let _ = fs::remove_file(source_zip_path);
    }

    println!("Deploy successful!");
    Ok(())
}
