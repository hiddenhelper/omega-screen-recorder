use anyhow::{Context, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub fps: Option<u32>,
    pub resolution: Option<String>,
    pub codec: Option<String>,
}

pub fn config_file_path() -> Result<PathBuf> {
    let base = config_dir().context("Could not determine user config directory")?;
    Ok(base.join("screenrec").join("config.toml"))
}

pub fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("Create config dir: {}", parent.display()))?;
    }
    Ok(())
}

pub fn load_config() -> Result<AppConfig> {
    let path = config_file_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let cfg: AppConfig = toml::from_str(&content)
        .with_context(|| {
            format!(
                "Failed to parse config file: {}\n\
                The file may be corrupted. You can delete it and run 'screenrec config' to recreate it.",
                path.display()
            )
        })?;
    Ok(cfg)
}

pub fn save_config(cfg: &AppConfig) -> Result<()> {
    let path = config_file_path()?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    ensure_parent_dir(&path)
        .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
    let content = toml::to_string_pretty(cfg)
        .context("Failed to serialize configuration to TOML format")?;
    fs::write(&path, content)
        .with_context(|| format!("Failed to write config file: {}", path.display()))?;
    Ok(())
}

pub fn clear_config() -> Result<()> {
    let path = config_file_path()?;
    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to delete config file: {}", path.display()))?;
        println!("Configuration cleared successfully.");
    } else {
        println!("No configuration file found. Nothing to clear.");
    }
    Ok(())
}


