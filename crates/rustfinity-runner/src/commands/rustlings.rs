use anyhow::{Context, Result};
use base64::prelude::*;
use std::fs;
use tempfile::TempDir;

use crate::utils::{emit, run_command_and_merge_output};

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

/// Runs a rustlings exercise with cargo test, streaming the output to stdout.
///
/// Returns whether the exercise passed.
pub async fn run_rustlings_test(params: &RustlingsParams) -> Result<bool> {
    let code = params.decode_code()?;
    let temp_dir = create_rustlings_project(&code)?;
    let cwd = path_str(&temp_dir)?;

    // Streams to stdout as cargo compiles and runs the tests.
    let result = run_command_and_merge_output("cargo", &["test", "--", "--nocapture"], Some(&cwd))
        .context("Failed to run cargo test")?;

    Ok(result.success)
}

/// Checks a rustlings exercise with cargo check (compilation only), streaming
/// the output to stdout, then runs it if it compiled.
///
/// Returns whether the exercise compiled and ran.
pub async fn run_rustlings_check(params: &RustlingsParams) -> Result<bool> {
    let code = params.decode_code()?;
    let temp_dir = create_rustlings_project(&code)?;
    let cwd = path_str(&temp_dir)?;

    let check = run_command_and_merge_output("cargo", &["check"], Some(&cwd))
        .context("Failed to run cargo check")?;

    if !check.success {
        return Ok(false);
    }

    // If check succeeded, also run the code to show output
    let header = "Compiling succeeded!\n\nOutput:\n";
    emit(header)?;

    let run = run_command_and_merge_output("cargo", &["run", "--quiet"], Some(&cwd))
        .context("Failed to run cargo run")?;

    Ok(run.success)
}

fn path_str(temp_dir: &TempDir) -> Result<String> {
    temp_dir
        .path()
        .to_str()
        .map(ToString::to_string)
        .context("Temp directory path is not valid UTF-8")
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
