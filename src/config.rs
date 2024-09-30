use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushProvider {
    pub name: String,
    pub provider_type: String,
    pub api_key: String,
    pub user_key: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub providers: Vec<PushProvider>,
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
                let mut config: Config = serde_yaml::from_str(&content)
                    .with_context(|| format!("Failed to parse config file: {:?}", path))?;
                
                // Set default base_url for Pushover providers if not specified
                for provider in &mut config.providers {
                    if provider.provider_type == "pushover" && provider.base_url.is_none() {
                        provider.base_url = Some("https://api.pushover.net/1/messages.json".to_string());
                    }
                }
                
                return Ok(config);
            }
        }

        Err(anyhow::anyhow!("No configuration file found"))
    }

    pub fn get_provider(&self, name: &str) -> Option<&PushProvider> {
        self.providers.iter().find(|p| p.name == name)
    }

    pub fn get_default_providers(&self) -> Vec<&PushProvider> {
        self.providers.iter().filter(|p| p.default).collect()
    }
}
