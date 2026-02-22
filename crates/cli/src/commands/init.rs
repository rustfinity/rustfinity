use anyhow::{bail, Result};
use include_dir::{include_dir, Dir};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

static AXUM_API: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/axum-api");

pub async fn init() -> Result<()> {
    let project_name = loop {
        print!("Project name: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let name = input.trim().to_string();

        if is_valid_package_name(&name) {
            break name;
        }
        println!("Invalid project name. Use lowercase letters, numbers, and hyphens (must start with a letter).");
    };

    let project_path = Path::new(&project_name);
    if project_path.exists() {
        let is_empty = project_path.read_dir()?.next().is_none();
        if !is_empty {
            bail!(
                "Directory '{}' already exists and is not empty.",
                project_name
            );
        }
    }

    write_template(&AXUM_API, project_path, &project_name)?;

    // Initialize git repo with initial commit
    let git_init = Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match git_init {
        Ok(s) if s.success() => {
            // Stage all files
            let add = Command::new("git")
                .args(["add", "-A"])
                .current_dir(project_path)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();

            if matches!(add, Ok(s) if s.success()) {
                // Create initial commit
                let commit = Command::new("git")
                    .args(["commit", "-m", "Initial commit"])
                    .current_dir(project_path)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();

                if !matches!(commit, Ok(s) if s.success()) {
                    eprintln!("Warning: Failed to create initial commit.");
                }
            } else {
                eprintln!("Warning: Failed to stage files for initial commit.");
            }
        }
        _ => eprintln!("Warning: Failed to initialize git repository."),
    }

    println!();
    println!("\u{2713} Created project \"{}\"", project_name);
    println!();
    println!("Next steps:");
    println!("  cd {}", project_name);
    println!("  cargo run");
    println!();
    println!("When ready to deploy:");
    println!("  rustfinity deploy");

    Ok(())
}

fn is_valid_package_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let first = name.chars().next().unwrap();
    if !first.is_ascii_lowercase() {
        return false;
    }

    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn write_template(dir: &Dir<'_>, dest: &Path, project_name: &str) -> Result<()> {
    for file in dir.files() {
        // Rename .template files back to their original names (e.g., Cargo.toml.template -> Cargo.toml)
        let file_path = if file.path().extension().map(|e| e == "template").unwrap_or(false) {
            dest.join(file.path().with_extension(""))
        } else {
            dest.join(file.path())
        };

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = file.contents();

        // Replace package name in Cargo.toml
        if file_path.file_name().map(|f| f == "Cargo.toml").unwrap_or(false) {
            let text = std::str::from_utf8(contents)?;
            let replaced = text.replace("name = \"my-app\"", &format!("name = \"{}\"", project_name));
            fs::write(&file_path, replaced)?;
        } else {
            fs::write(&file_path, contents)?;
        }
    }

    for subdir in dir.dirs() {
        write_template(subdir, dest, project_name)?;
    }

    Ok(())
}
