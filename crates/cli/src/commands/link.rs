use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct ProjectConfig {
    project_id: String,
    name: String,
}

pub async fn link(project_id: &str) -> Result<()> {
    let config_path = Path::new("rustfinity.json");
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
