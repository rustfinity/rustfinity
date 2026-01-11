use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about = "CLI for rust code runner", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Run the code based on the code, tests, and cargo toml file provided")]
    Test {
        /// Code base64 encoded
        #[clap(long)]
        code: String,

        /// Tests base64 encoded
        #[clap(long)]
        tests: String,

        /// Cargo toml base64 encoded
        #[clap(long)]
        cargo_toml: String,

        #[clap(long = "n-tests", short)]
        /// number of tests to take the minimum time of
        n_tests: Option<usize>,
    },

    #[clap(about = "Run and test the code based on the challenge and code provided")]
    Playground {
        /// Code base64 encoded
        #[clap(long)]
        code: String,
    },

    #[clap(about = "Run rustlings exercise with tests (cargo test)")]
    RustlingsTest {
        /// Code base64 encoded
        #[clap(long)]
        code: String,
    },

    #[clap(about = "Check rustlings exercise compilation only (cargo check)")]
    RustlingsCheck {
        /// Code base64 encoded
        #[clap(long)]
        code: String,
    },
}
