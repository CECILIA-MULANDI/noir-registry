use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub registry_url: Option<String>,
}
impl Config {
    /// Get the path to the config file
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .context("Could not find config directory")?;
        let noir_registry_dir = config_dir.join("noir-registry");
        fs::create_dir_all(&noir_registry_dir).context("Failed to create config directory")?;

        Ok(noir_registry_dir.join("config.toml"))
    }
    /// Load config from file
    pub fn load() -> Result<Config> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&path).context("Failed to read config file")?;

        toml::from_str(&content)
            .context("Failed to parse config file")
            .map_err(Into::into)
    }
    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&path, content).context("Failed to write config file")?;

        Ok(())
    }

    /// Get API key from config
    pub fn get_api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Set API key in config
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }

    /// Set registry URL in config
    pub fn set_registry_url(&mut self, registry_url: String) {
        self.registry_url = Some(registry_url);
    }
}
