use base64::prelude::*;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use crate::constants::PLAYGROUND_DIR;
use crate::regex::extract_unittest_path;
use crate::utils::{emit, run_command_and_merge_output, write_file};

pub struct RunTestsParams {
    code_base64: String,
    tests_base64: String,
    cargo_toml_base64: String,
    n_tests: usize,
}

impl RunTestsParams {
    pub fn new(
        code_base64: String,
        tests_base64: String,
        cargo_toml_base64: String,
        n_tests: Option<usize>,
    ) -> Self {
        Self {
            code_base64,
            n_tests: n_tests.unwrap_or(1),
            tests_base64,
            cargo_toml_base64,
        }
    }
}

pub async fn run_tests(params: &RunTestsParams) -> anyhow::Result<String> {
    let RunTestsParams {
        code_base64,
        n_tests,
        tests_base64,
        cargo_toml_base64,
    } = params;

    // Already streamed to stdout line by line; kept here for the benchmarks.
    let mut output = execute_code(&code_base64, &tests_base64, &cargo_toml_base64).await?;

    let test_binary_path = extract_unittest_path(&output);

    if let Some(test_binary_path) = test_binary_path {
        let time_output = benchmark_time_min(&test_binary_path, n_tests).await?;
        let memory_output = memory_benchmark(&test_binary_path).await?;

        let benchmarks = format!("\n---\n{}\n{}\n", time_output, memory_output);
        emit(&benchmarks)?;
        output.push_str(&benchmarks);
    }

    Ok(output)
}

async fn benchmark_time(test_binary_path: &str) -> anyhow::Result<f64> {
    let cwd = std::env::var("PROJECT_PATH").unwrap_or(PLAYGROUND_DIR.to_string());

    let start = Instant::now();

    Command::new(&test_binary_path).current_dir(&cwd).output()?;

    let elapsed = start.elapsed();
    let as_nanos = elapsed.as_nanos();

    let as_ms = as_nanos as f64 / 1_000_000.0;

    Ok(as_ms)
}

/// Runs the tests 10 times and gets the minimum time
async fn benchmark_time_min(test_binary_path: &str, n_tests: &usize) -> anyhow::Result<String> {
    let mut nums = Vec::with_capacity(10);

    for _ in 0..*n_tests {
        let time = benchmark_time(&test_binary_path).await?;
        nums.push(time);
    }

    // Take Average
    let min = nums
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let final_output = format!("Time: {:.8}ms", min);

    Ok(final_output)
}

async fn memory_benchmark(test_binary_path: &str) -> anyhow::Result<String> {
    let cwd = std::env::var("PROJECT_PATH").unwrap_or(PLAYGROUND_DIR.to_string());

    let output = Command::new("heaptrack")
        .arg(&test_binary_path)
        .current_dir(&cwd)
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    let output_path = stdout.split("\"").collect::<Vec<&str>>()[1];

    // run heaptrack --analyze {output_path}
    let output = Command::new("heaptrack")
        .arg("--analyze")
        .arg(output_path)
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    let details = stdout.split("\n\n").collect::<Vec<&str>>();
    let last_index = details.len() - 1;
    let details = details[last_index];

    let mut memory = String::new();

    details.lines().for_each(|line| {
        if line.contains("peak heap memory consumption:") {
            memory = line.replace("peak", "Peak");
        }
    });

    Ok(memory)
}

fn to_utf8(base64: &str) -> anyhow::Result<String> {
    let utf8 = BASE64_STANDARD.decode(base64)?;
    Ok(String::from_utf8(utf8)?)
}

async fn execute_code(
    code_base64: &str,
    tests_base64: &str,
    config_toml_base64: &str,
) -> anyhow::Result<String> {
    let code = to_utf8(code_base64)?;
    let tests = to_utf8(tests_base64)?;
    let config_toml = to_utf8(config_toml_base64)?;

    let cwd = std::env::var("PROJECT_PATH").unwrap_or(PLAYGROUND_DIR.to_string());
    let tests_path = Path::new(&cwd).join("tests/tests.rs");
    let config_toml_path = Path::new(&cwd).join("Cargo.toml");
    let lib_path = Path::new(&cwd).join("src/lib.rs");

    // Write src/lib.rs
    write_file(&lib_path, &code)?;
    // Write tests/tests.rs
    write_file(&tests_path, &tests)?;
    // Write Cargo.toml
    write_file(&config_toml_path, &config_toml)?;

    let result = run_command_and_merge_output("cargo", &["test"], Some(&cwd))?;

    Ok(result.output)
}
