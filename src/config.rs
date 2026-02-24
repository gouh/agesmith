use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Spanish,
    English,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary: (u8, u8, u8),
    pub secondary: (u8, u8, u8),
    pub success: (u8, u8, u8),
    pub error: (u8, u8, u8),
    pub warning: (u8, u8, u8),
    pub bg: (u8, u8, u8),
    pub fg: (u8, u8, u8),
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            primary: (129, 212, 250),
            secondary: (171, 71, 188),
            success: (102, 187, 106),
            error: (239, 83, 80),
            warning: (255, 167, 38),
            bg: (38, 50, 56),
            fg: (255, 255, 255),
        }
    }

    pub fn light() -> Self {
        Self {
            primary: (25, 118, 210),
            secondary: (123, 31, 162),
            success: (56, 142, 60),
            error: (211, 47, 47),
            warning: (245, 124, 0),
            bg: (250, 250, 250),
            fg: (33, 33, 33),
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "light" => Self::light(),
            _ => Self::dark(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub auto_lock_minutes: u64,
    #[serde(alias = "clipboard_clear_seconds")]
    pub message_timeout_seconds: u64,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "en".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            auto_lock_minutes: 15,
            message_timeout_seconds: 3,
            language: "en".to_string(),
        }
    }
}

impl Config {
    pub fn get_language(&self) -> Language {
        match self.language.as_str() {
            "en" => Language::English,
            _ => Language::Spanish,
        }
    }

    pub fn get_theme(&self) -> Theme {
        Theme::from_name(&self.theme)
    }
}

pub fn load_config() -> Result<Config> {
    let config_path = dirs::home_dir()
        .context("Could not get home directory")?
        .join(".config/agesmith/config.toml");

    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Ok(Config::default())
    }
}

pub fn load_favorites() -> Result<Vec<PathBuf>> {
    let fav_path = dirs::home_dir()
        .context("Could not get home directory")?
        .join(".config/agesmith/favorites.json");

    if fav_path.exists() {
        let content = fs::read_to_string(&fav_path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(Vec::new())
    }
}

pub fn save_favorites(favorites: &[PathBuf]) -> Result<()> {
    let fav_path = dirs::home_dir()
        .context("Could not get home directory")?
        .join(".config/agesmith/favorites.json");

    if let Some(parent) = fav_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(favorites)?;
    fs::write(&fav_path, content)?;
    Ok(())
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = dirs::home_dir()
        .context("Could not get home directory")?
        .join(".config/agesmith/config.toml");

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}
