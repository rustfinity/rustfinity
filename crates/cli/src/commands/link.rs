use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    project_id: String,
    name: String,
}

pub async fn link(project_id: &str) -> Result<()> {
    // 1. Check if rustfinity.json already exists
    let config_path = Path::new("rustfinity.json");
    if config_path.exists() {
        let contents = fs::read_to_string(config_path)
            .context("Failed to read rustfinity.json")?;
        let existing: ProjectConfig = serde_json::from_str(&contents)
            .context("Failed to parse rustfinity.json")?;
        bail!(
            "This directory is already linked to project '{}' (ID: {}).\n\
             To re-link, remove rustfinity.json first.",
            existing.name,
            existing.project_id
        );
    }

    // 2. Write rustfinity.json (skip project existence check — validated on deploy)
    let project_config = ProjectConfig {
        project_id: project_id.to_string(),
        name: project_id.to_string(),
    };
    let config_json = serde_json::to_string_pretty(&project_config)
        .context("Failed to serialize project config")?;
    fs::write(config_path, format!("{config_json}\n"))
        .context("Failed to write rustfinity.json")?;

    println!("\x1b[32mLinked to project (ID: {})\x1b[0m", project_id);
    println!("You can now run `rustfinity deploy` to deploy to this project.");

    Ok(())
}
