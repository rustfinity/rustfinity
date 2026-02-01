# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Rustfinity is an interactive learning platform for Rust developers. The repository contains:

- **challenges/**: Individual coding exercises with starter code, solutions, and tests
- **crates/cli**: CLI tool for downloading and submitting challenges locally
- **crates/rustfinity-runner**: Test runner that executes code in secure Docker containers
- **crates/syntest**: Syntax-based testing library using Rust AST analysis

## Build and Test Commands

### Workspace-level

```bash
# Build entire workspace
cargo build

# Build with release optimizations (uses LTO)
cargo build --release

# Run all workspace tests
cargo test

# Run challenge validation tests (ensures all challenges have required files)
cd challenges && cargo test
```

### Challenge-specific

```bash
# Test a specific challenge
cargo test -p <challenge-name>

# Example: Test the fibonacci challenge
cargo test -p fibonacci

# Run a single test file
cargo test --test tests -p fibonacci
```

### CLI crate

```bash
# Build the CLI
cargo build -p rustfinity

# Run CLI commands
cargo run -p rustfinity -- get challenge <challenge-slug>
```

### Runner crate

```bash
# The rustfinity-runner is designed to run in Docker
# It accepts base64-encoded code, tests, and Cargo.toml
cargo run -p rustfinity-runner -- test --code <base64> --tests <base64> --cargo-toml <base64>
```

## Repository Structure

### Challenge Structure

Each challenge in `challenges/<challenge-name>/` follows this strict structure:

- **description.md**: Challenge instructions and examples
- **Cargo.toml**: Package definition (package name must match directory name)
- **src/lib.rs**: Complete solution implementation
- **src/starter.rs**: Starter code template for users (incomplete solution)
- **tests/tests.rs**: Test cases that validate the solution

The `challenges/tests.rs` file validates that all challenges follow this structure.

### Workspace Organization

The workspace uses Cargo's resolver v2 and includes:

- `challenges` - Parent crate that provides shared utilities
- `challenges/*` - Individual challenge packages
- `crates/*` - Supporting tools (CLI, runner, syntest)

### Key Files

- **challenges/lib.rs**: Utilities for reading and validating challenges
  - `challenges_json()`: Parses challenges.json metadata
  - `challenges_dir_list()`: Lists all challenge directories
  - `get_max_id()`: Returns highest challenge ID
- **challenges/challenges.json**: Metadata for all challenges (id, title, slug, difficulty, track, tags, dates)
- **challenges/cli.rs**: CLI tool for challenge management

## Challenge Metadata System

All challenges are registered in `challenges/challenges.json` with:

- Unique `id` (numeric, validated for duplicates)
- `slug` (must match directory name)
- `difficulty`: BEGINNER, EASY, MEDIUM, HARD, ADVANCED
- `track`: RUST_BASICS, CONTROL_FLOW, DSA
- `tags`: Array of topic tags
- `created_at` and `updated_at`: ISO 8601 timestamps (validated)

### Adding a New Challenge

1. Create directory under `challenges/<slug>/`
2. Add required files: description.md, Cargo.toml, src/lib.rs, src/starter.rs, tests/tests.rs
3. Ensure Cargo.toml package name matches directory slug
4. Add entry to challenges/challenges.json with unique ID (use `challenges::get_max_id()` + 1)
5. Run `cargo test -p challenges` to validate structure

## Testing Architecture

### Test Types

1. **Challenge tests** (`challenges/*/tests/tests.rs`): Validate challenge solutions
2. **Structure tests** (`challenges/tests.rs`): Validate all challenges have required files and metadata
3. **Syntest**: AST-based syntax validation for advanced testing scenarios

### Running Tests

- Challenge structure validation catches common issues:
  - Missing required files
  - Mismatched package names
  - Duplicate IDs or slugs
  - Invalid timestamps
  - Missing metadata entries

## CLI Usage

The `rustfinity` CLI allows users to download challenges locally:

```bash
# Download a specific challenge
rustfinity get challenge <challenge-slug>

# The CLI checks for updates on each run and prompts users to upgrade if outdated
```

## Crate Dependencies

### cli

- Uses `clap` for command-line parsing
- `reqwest` for HTTP requests (with rustls-tls)
- `tokio` async runtime
- Downloads challenges from GitHub repository

### rustfinity-runner

- Designed for secure, isolated test execution in Docker
- Uses `duct` for process management
- Base64 encoding for code/test transport
- Supports both regular challenges and Rustlings exercises

### syntest

- AST-based syntax testing using `syn` crate
- Allows testing code structure without execution
- Use `create_bin_and_run()` to compile and execute code snippets

## Development Workflow

When modifying challenges:

1. Edit solution in `src/lib.rs`
2. Update starter template in `src/starter.rs` (should be incomplete)
3. Verify tests in `tests/tests.rs` pass with solution
4. Update `description.md` if requirements change
5. Update `updated_at` in challenges.json
6. Run `cargo test -p challenges` to validate structure
7. Run `cargo test -p <challenge-name>` to verify tests pass

## Scripts

- **scripts/run-challenge-in-docker.py**: Utility for running challenges in Docker environment
