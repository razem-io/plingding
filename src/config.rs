use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub user_key: String,
    pub base_url: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_paths = vec![
            dirs::home_dir().map(|p| p.join(".plingding.yaml")),
            dirs::home_dir().map(|p| p.join(".config/plingding/plingding.yaml")),
            Some(PathBuf::from("plingding.yaml")),
        ];

        for path in config_paths.into_iter().flatten() {
            if path.exists() {
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read config file: {:?}", path))?;
                let config: Config = serde_yaml::from_str(&content)
                    .with_context(|| format!("Failed to parse config file: {:?}", path))?;
                return Ok(config);
            }
        }

        Err(anyhow::anyhow!("No configuration file found"))
    }
}
