use std::path::Path;

use crate::{
    constants::PLAYGROUND_DIR,
    utils::{run_command_and_merge_output, to_utf8, write_file},
};

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

    // Write src/main.rs
    write_file(&main_path, &code)?;
    let result = run_command_and_merge_output("cargo", &["run"], Some(&cwd))?;

    Ok(result.output)
}
