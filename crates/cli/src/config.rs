use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home.join(".config").join("rustfinity-cloud").join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Could not read config file at {}", path.display()))?;
        let config: Config =
            serde_json::from_str(&contents).context("Could not parse config file")?;
        Ok(config)
    }

    pub fn save(api_key: &str) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Could not create config directory {}", parent.display()))?;
        }
        let config = Config {
            api_key: api_key.to_string(),
        };
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(&path, json)
            .with_context(|| format!("Could not write config file to {}", path.display()))?;
        Ok(())
    }

    pub fn delete() -> Result<bool> {
        let path = Self::config_path()?;
        if path.exists() {
            fs::remove_file(&path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
