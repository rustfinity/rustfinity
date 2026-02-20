use anyhow::{bail, Context, Result};
use reqwest::{multipart, StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::auth;
use crate::confirm::confirm;
use crate::constants::api_base_url;

#[derive(Debug)]
enum DeployError {
    HttpError { status: StatusCode, body: String },
    Other(anyhow::Error),
}

impl std::fmt::Display for DeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeployError::HttpError { status, body } => {
                write!(f, "Deploy failed (HTTP {}): {}", status, body)
            }
            DeployError::Other(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for DeployError {}

impl From<anyhow::Error> for DeployError {
    fn from(e: anyhow::Error) -> Self {
        DeployError::Other(e)
    }
}

#[derive(Deserialize)]
struct DeployResponse {
    project_id: String,
    deployment_id: String,
    status: String,
    is_new_project: bool,
    #[allow(dead_code)]
    subdomain: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    project_id: String,
    name: String,
}

#[derive(Deserialize)]
struct CargoTomlPackage {
    name: String,
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
}

const TARGET: &str = "x86_64-unknown-linux-gnu";

/// Check if the current directory is inside a git repository.
/// If not, offer to initialize one for the user.
fn ensure_git_repo() -> Result<()> {
    // Check if git is installed
    let git_installed = Command::new("git")
        .args(["--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !git_installed {
        bail!(
            "Git is not installed. Rustfinity deploy requires git to create source archives.\n\
             Please install git: https://git-scm.com/downloads"
        );
    }

    // Check if we're inside a git repo
    let in_git_repo = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if in_git_repo {
        return Ok(());
    }

    println!(
        "\x1b[33mThis directory is not a git repository.\x1b[0m"
    );
    println!(
        "Rustfinity deploy uses git to create source archives of your project."
    );

    let yes = confirm(
        "Would you like to initialize a git repository here?",
        true,
    )
    .context("Failed to read input")?;

    if !yes {
        bail!(
            "A git repository is required for deployment.\n\
             You can initialize one manually with: git init && git add -A && git commit -m \"Initial commit\""
        );
    }

    // git init
    let status = Command::new("git")
        .args(["init"])
        .status()
        .context("Failed to run git init")?;
    if !status.success() {
        bail!("git init failed");
    }

    // git add -A
    let status = Command::new("git")
        .args(["add", "-A"])
        .status()
        .context("Failed to run git add")?;
    if !status.success() {
        bail!("git add failed");
    }

    // git commit
    let status = Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .status()
        .context("Failed to run git commit")?;
    if !status.success() {
        bail!("git commit failed");
    }

    println!("\x1b[32mGit repository initialized successfully.\x1b[0m");
    Ok(())
}

fn build_for_target() -> Result<()> {
    let target = TARGET;

    if std::env::consts::OS == "linux" {
        // On Linux, plain cargo build works — no cross toolchain needed
        let status = Command::new("cargo")
            .args(["build", "--release", "--target", target])
            .status()
            .context("Failed to run cargo build")?;
        if !status.success() {
            bail!("cargo build failed with exit code: {}", status);
        }
    } else {
        // Non-Linux: need zig + cargo-zigbuild for cross-compilation
        // Check zig first — we can't install it for the user
        if !Command::new("zig")
            .arg("version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            bail!(
                "\x1b[33mYou're on {}, cross-compiling to Linux requires zig.\n\
                 Please install zig first: https://ziglang.org/download/\x1b[0m",
                std::env::consts::OS
            );
        }

        // Check cargo-zigbuild — offer to install if missing
        if !Command::new("cargo-zigbuild")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            let yes = confirm(
                "\x1b[33mcargo-zigbuild is not installed. Install it now?\x1b[0m",
                true,
            )
            .context("Failed to read input")?;
            if !yes {
                bail!("cargo-zigbuild is required. Install it with: cargo install cargo-zigbuild");
            }
            println!("Installing cargo-zigbuild...");
            let install = Command::new("cargo")
                .args(["install", "cargo-zigbuild"])
                .status()
                .context("Failed to run cargo install cargo-zigbuild")?;
            if !install.success() {
                bail!("Failed to install cargo-zigbuild");
            }
        }

        let status = Command::new("cargo")
            .args(["zigbuild", "--release", "--target", target])
            .status()
            .context("Failed to run cargo zigbuild")?;
        if !status.success() {
            bail!("cargo zigbuild failed with exit code: {}", status);
        }
    }

    Ok(())
}

fn is_auth_error(e: &DeployError) -> bool {
    matches!(
        e,
        DeployError::HttpError { status, .. } if status.as_u16() == 401
    )
}

fn is_project_not_found_error(e: &DeployError) -> bool {
    match e {
        DeployError::HttpError { status, body } => {
            status.as_u16() == 400 && body.contains("Project not found")
        }
        _ => false,
    }
}

fn delete_rustfinity_json() -> Result<()> {
    let path = Path::new("rustfinity.json");
    if path.exists() {
        fs::remove_file(path).context("Failed to remove rustfinity.json")?;
    }
    Ok(())
}

pub async fn deploy() -> Result<()> {
    deploy_with_retry().await
}

async fn deploy_with_retry() -> Result<()> {
    // Track if we've already retried for each error type to avoid infinite loops
    let mut auth_retried = false;
    let mut project_not_found_retried = false;

    loop {
        match deploy_internal().await {
            Ok(()) => return Ok(()),
            Err(e) => {
                // Check if it's a 401 auth error
                if is_auth_error(&e) && !auth_retried {
                    println!("Authentication failed. Logging in...");
                    auth::perform_login().await?;
                    println!("Retrying deploy...");
                    auth_retried = true;
                    continue;
                }

                // Check if it's a deleted project error (400 + "Project not found")
                if is_project_not_found_error(&e) && !project_not_found_retried {
                    println!("Project not found (may have been deleted). Creating new project...");
                    delete_rustfinity_json()?;
                    println!("Retrying deploy...");
                    project_not_found_retried = true;
                    continue;
                }

                // Other errors or already retried - propagate
                return Err(e.into());
            }
        }
    }
}

async fn deploy_internal() -> Result<(), DeployError> {
    // Helper to convert anyhow errors to DeployError
    let to_deploy_error = |e: anyhow::Error| DeployError::Other(e);

    // 1. Load config (check auth)
    let config = auth::ensure_authenticated().await.map_err(to_deploy_error)?;

    // 2. Verify Cargo.toml exists
    let cargo_toml_path = Path::new("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err(DeployError::Other(anyhow::anyhow!(
            "No Cargo.toml found in the current directory. Please run this command from a Rust project root."
        )));
    }

    // 3. Ensure we're in a git repo (offer to initialize if not)
    ensure_git_repo().map_err(to_deploy_error)?;

    // 4. Parse Cargo.toml to get package name and derive slug
    let cargo_toml_contents = fs::read_to_string(cargo_toml_path)
        .context("Failed to read Cargo.toml")
        .map_err(to_deploy_error)?;
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_contents)
        .context("Failed to parse Cargo.toml")
        .map_err(to_deploy_error)?;
    let package_name = &cargo_toml.package.name;
    let slug = package_name.replace('_', "-");

    println!("Deploying project: {} (slug: {})", package_name, slug);

    // 5. Load rustfinity.json if it exists (get project_id)
    let project_config_path = Path::new("rustfinity.json");
    let existing_project_id = if project_config_path.exists() {
        let contents = fs::read_to_string(project_config_path)
            .context("Failed to read .rustfinity.json")
            .map_err(to_deploy_error)?;
        let project_config: ProjectConfig = serde_json::from_str(&contents)
            .context("Failed to parse .rustfinity.json")
            .map_err(to_deploy_error)?;
        Some(project_config.project_id)
    } else {
        None
    };

    // 6. Build release binary for x86_64-unknown-linux-gnu
    println!("Building release binary...");
    build_for_target().map_err(to_deploy_error)?;

    // 7. Locate binary
    let binary_path = format!(
        "target/x86_64-unknown-linux-gnu/release/{}",
        package_name
    );
    let binary_path = Path::new(&binary_path);
    if !binary_path.exists() {
        return Err(DeployError::Other(anyhow::anyhow!(
            "Expected binary not found at {}. Make sure the package produces a binary target.",
            binary_path.display()
        )));
    }

    // 8. Determine upload filename
    let binary_suffix = match &existing_project_id {
        Some(project_id) => project_id.clone(),
        None => slug.clone(),
    };
    let binary_name = format!("rustfinity-app-{}", binary_suffix);

    // 9. Create source zip via git archive
    println!("Creating source archive...");
    let source_zip_path = "target/release/rustfinity-source.zip";
    let archive_status = Command::new("git")
        .args([
            "archive",
            "--format=zip",
            &format!("--output={}", source_zip_path),
            "HEAD",
        ])
        .status()
        .context("Failed to run git archive")
        .map_err(to_deploy_error)?;

    if !archive_status.success() {
        return Err(DeployError::Other(anyhow::anyhow!(
            "Failed to create source archive via git archive. Make sure you have at least one commit."
        )));
    }

    // 10. Create multipart form and send request
    println!("Uploading to Rustfinity Cloud...");
    let binary_bytes = fs::read(binary_path)
        .context("Failed to read binary file")
        .map_err(to_deploy_error)?;

    let binary_part = multipart::Part::bytes(binary_bytes)
        .file_name(binary_name.clone())
        .mime_str("application/octet-stream")
        .map_err(|e| DeployError::Other(anyhow::anyhow!("Failed to set mime type: {}", e)))?;

    let mut form = multipart::Form::new()
        .part("binary", binary_part)
        .text("project_name", package_name.clone())
        .text("name", slug.clone());

    if let Some(ref project_id) = existing_project_id {
        form = form.text("project_id", project_id.clone());
    }

    form = form.text("target", TARGET.to_string());

    let source_bytes = fs::read(source_zip_path)
        .context("Failed to read source zip")
        .map_err(to_deploy_error)?;
    let source_part = multipart::Part::bytes(source_bytes)
        .file_name("rustfinity-source.zip")
        .mime_str("application/zip")
        .map_err(|e| DeployError::Other(anyhow::anyhow!("Failed to set mime type: {}", e)))?;
    form = form.part("source_zip", source_part);

    // 11. POST to deploy endpoint
    let base_url = api_base_url();
    let url = format!("{}/deploy", base_url);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .multipart(form)
        .send()
        .await
        .context("Failed to send deploy request")
        .map_err(to_deploy_error)?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(DeployError::HttpError { status, body });
    }

    let deploy_response: DeployResponse = response
        .json()
        .await
        .context("Failed to parse deploy response")
        .map_err(to_deploy_error)?;

    // 12. Save project config to .rustfinity.json
    let project_config = ProjectConfig {
        project_id: deploy_response.project_id.clone(),
        name: slug.clone(),
    };
    let config_json = serde_json::to_string_pretty(&project_config)
        .context("Failed to serialize project config")
        .map_err(to_deploy_error)?;
    fs::write(project_config_path, config_json)
        .context("Failed to write .rustfinity.json")
        .map_err(to_deploy_error)?;

    // 13. Print deployment result
    if deploy_response.is_new_project {
        println!("Created new project: {}", slug);
        println!();
        println!("Tip: Make sure your application listens on the PORT environment variable (defaults to 3000).");
    } else {
        println!("Redeployed project: {}", slug);
    }
    println!("Deployment ID: {}", deploy_response.deployment_id);
    println!("Status: {}", deploy_response.status);
    println!("URL: {}", deploy_response.url);

    println!("Deploy successful!");
    Ok(())
}
