use std::path::Path;

use crate::{
    constants::PLAYGROUND_DIR,
    utils::{run_command_and_merge_output, to_utf8, write_file},
};

/// The image's playground manifest lists syn, quote, tempfile and tokio so the
/// challenge tests have them prebuilt. `cargo run` would build every one of
/// them (a different feature set than the prebuilt one, so nothing is reused)
/// for a program that uses none, so the playground runs with no dependencies.
const PLAYGROUND_MANIFEST: &str = r#"[package]
name = "playground"
version = "0.1.0"
edition = "2024"

[dependencies]
"#;

pub struct PlaygroundParams {
    code_base64: String,
}

impl PlaygroundParams {
    pub fn new(code_base64: String) -> Self {
        Self { code_base64 }
    }
}

pub async fn run_code_in_playground(params: &PlaygroundParams) -> anyhow::Result<String> {
    let PlaygroundParams { code_base64 } = params;

    // Already streamed to stdout line by line as cargo produced it.
    execute_code(&code_base64).await
}

async fn execute_code(code_base64: &str) -> anyhow::Result<String> {
    let code = to_utf8(code_base64)?;

    let cwd = std::env::var("PROJECT_PATH").unwrap_or(PLAYGROUND_DIR.to_string());
    let main_path = Path::new(&cwd).join("src/main.rs");

    // Write Cargo.toml without dependencies, then src/main.rs
    write_file(&Path::new(&cwd).join("Cargo.toml"), PLAYGROUND_MANIFEST)?;
    write_file(&main_path, &code)?;
    let result = run_command_and_merge_output("cargo", &["run"], Some(&cwd))?;

    Ok(result.output)
}
