use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Creates `dir` (and parents). On Unix the leaf directory is restricted to
/// the current user (0700), since it holds the API key.
fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

/// Writes `contents` to `path` so that only the current user can read it.
///
/// The file is created with mode 0600 on Unix, and an existing file's mode is
/// tightened before writing, so the API key is never readable by other local
/// users (the default umask would otherwise leave it world-readable).
fn write_private_file(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
    }
    file.write_all(contents)?;
    file.sync_all()
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home
            .join(".config")
            .join("rustfinity-cloud")
            .join("config.json"))
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
            create_private_dir(parent).with_context(|| {
                format!("Could not create config directory {}", parent.display())
            })?;
        }
        let config = Config {
            api_key: api_key.to_string(),
        };
        let json = serde_json::to_string_pretty(&config)?;
        write_private_file(&path, json.as_bytes())
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

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn private_file_is_owner_only() {
        let dir = tempfile::tempdir().unwrap();
        let conf_dir = dir.path().join("rustfinity-cloud");
        create_private_dir(&conf_dir).unwrap();
        let path = conf_dir.join("config.json");

        // A pre-existing world-readable file must be tightened too.
        fs::write(&path, "old").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

        write_private_file(&path, b"secret").unwrap();

        let file_mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        let dir_mode = fs::metadata(&conf_dir).unwrap().permissions().mode() & 0o777;
        assert_eq!(file_mode, 0o600);
        assert_eq!(dir_mode, 0o700);
        assert_eq!(fs::read_to_string(&path).unwrap(), "secret");
    }
}
