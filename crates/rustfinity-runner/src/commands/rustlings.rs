use anyhow::{Context, Result};
use base64::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Parameters for running a rustlings exercise
pub struct RustlingsParams {
    pub code: String,
}

impl RustlingsParams {
    pub fn new(code_base64: String) -> Self {
        Self { code: code_base64 }
    }

    fn decode_code(&self) -> Result<String> {
        let bytes = BASE64_STANDARD
            .decode(&self.code)
            .context("Failed to decode code from base64")?;
        String::from_utf8(bytes).context("Code is not valid UTF-8")
    }
}

/// Run rustlings exercise with cargo test
pub async fn run_rustlings_test(params: &RustlingsParams) -> Result<String> {
    let code = params.decode_code()?;
    let temp_dir = create_rustlings_project(&code)?;

    // Run cargo test
    let output = Command::new("cargo")
        .args(["test", "--", "--nocapture"])
        .current_dir(temp_dir.path())
        .output()
        .context("Failed to run cargo test")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    Ok(format!("{}{}", stderr, stdout))
}

/// Run rustlings exercise with cargo check (compilation only)
pub async fn run_rustlings_check(params: &RustlingsParams) -> Result<String> {
    let code = params.decode_code()?;
    let temp_dir = create_rustlings_project(&code)?;

    // Run cargo check
    let output = Command::new("cargo")
        .args(["check"])
        .current_dir(temp_dir.path())
        .output()
        .context("Failed to run cargo check")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // If check succeeded, also run the code to show output
    if output.status.success() {
        let run_output = Command::new("cargo")
            .args(["run", "--quiet"])
            .current_dir(temp_dir.path())
            .output()
            .context("Failed to run cargo run")?;

        let run_stdout = String::from_utf8_lossy(&run_output.stdout);
        let run_stderr = String::from_utf8_lossy(&run_output.stderr);

        if run_output.status.success() {
            return Ok(format!(
                "{}{}Compiling succeeded!\n\nOutput:\n{}{}",
                stderr, stdout, run_stderr, run_stdout
            ));
        }
    }

    Ok(format!("{}{}", stderr, stdout))
}

/// Create a temporary Cargo project for the rustlings exercise
fn create_rustlings_project(code: &str) -> Result<TempDir> {
    let temp_dir = TempDir::new().context("Failed to create temp directory")?;
    let project_path = temp_dir.path();

    // Create src directory
    fs::create_dir(project_path.join("src")).context("Failed to create src directory")?;

    // Write main.rs with the user code
    fs::write(project_path.join("src/main.rs"), code).context("Failed to write main.rs")?;

    // Write Cargo.toml
    let cargo_toml = r#"[package]
name = "rustlings_exercise"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
    fs::write(project_path.join("Cargo.toml"), cargo_toml).context("Failed to write Cargo.toml")?;

    Ok(temp_dir)
}
