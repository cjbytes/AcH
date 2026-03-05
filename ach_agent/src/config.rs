use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend: Backend,
    pub api_key: Option<String>,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Backend {
    Ollama,
    OpenAI,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backend: Backend::Ollama,
            api_key: None,
            model: "llama2".to_string(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("aichat/config.toml")
    }
    pub fn load_or_create_default() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path();
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            Ok(toml::from_str(&data)?)
        } else {
            let config = Self::default();
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, toml::to_string(&config)?)?;
            Ok(config)
        }
    }
}
