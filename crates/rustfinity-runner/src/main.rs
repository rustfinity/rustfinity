use clap::Parser;
use cli::{Cli, Commands};
use commands::{
    playground::{run_code_in_playground, PlaygroundParams},
    run_tests::{run_tests, RunTestsParams},
    rustlings::{run_rustlings_check, run_rustlings_test, RustlingsParams},
};
use dotenvy::dotenv;
use std::process::ExitCode;

mod cli;
mod commands;
mod constants;
mod regex;
mod utils;

#[tokio::main]
async fn main() -> ExitCode {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Test {
            code: code_base64,
            tests: tests_base64,
            cargo_toml: cargo_toml_base64,
            n_tests,
        } => {
            let params = RunTestsParams::new(code_base64, tests_base64, cargo_toml_base64, n_tests);

            match run_tests(&params).await {
                Ok(output) => {
                    println!("{}", output);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }

        Commands::Playground { code: code_base64 } => {
            let params = PlaygroundParams::new(code_base64);

            match run_code_in_playground(&params).await {
                Ok(output) => {
                    println!("{}", output);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }

        Commands::RustlingsTest { code: code_base64 } => {
            let params = RustlingsParams::new(code_base64);

            match run_rustlings_test(&params).await {
                Ok(result) => {
                    println!("{}", result.output);
                    if result.success {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }

        Commands::RustlingsCheck { code: code_base64 } => {
            let params = RustlingsParams::new(code_base64);

            match run_rustlings_check(&params).await {
                Ok(result) => {
                    println!("{}", result.output);
                    if result.success {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }
    }
}
