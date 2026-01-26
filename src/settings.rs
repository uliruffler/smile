//! Settings management for Smile emoticon picker
//!
//! This module handles loading and saving settings from ~/.smile/settings.toml
//! and recently used emoticons from ~/.smile/recent.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Default settings file content
const DEFAULT_SETTINGS_TOML: &str = r#"# Smile Emoticon Picker Settings
# This file is located at ~/.smile/settings.toml

# Maximum number of recently used emoticons to keep
max_recent = 10

# Keywords for emoticon categories
# These keywords are used for searching emoticons
[keywords]

[keywords.happy]
terms = ["happy", "joy", "smile", "grin", "cheerful", "glad", "pleased"]

[keywords.sad]
terms = ["sad", "cry", "unhappy", "depressed", "down", "tear", "weep"]

[keywords.angry]
terms = ["angry", "mad", "furious", "rage", "annoyed", "irritated"]

[keywords.surprised]
terms = ["surprised", "shocked", "amazed", "astonished", "wow"]

[keywords.love]
terms = ["love", "heart", "romance", "affection", "adore", "crush"]

[keywords.gestures]
terms = ["hand", "gesture", "thumbs", "clap", "wave", "point", "fist", "ok", "peace"]

[keywords.faces]
terms = ["face", "expression", "neutral", "tired", "sleepy", "bored"]

[keywords.cool]
terms = ["cool", "awesome", "sunglasses", "cat", "kitty", "nerd", "smart"]

[keywords.symbols]
terms = ["star", "fire", "sparkle", "lightning", "check", "cross", "rainbow"]

[keywords.objects]
terms = ["party", "celebration", "gift", "trophy", "medal", "award", "balloon"]

[keywords.animals]
terms = ["animal", "pet", "dog", "cat", "mouse", "bear", "fox", "lion", "monkey"]

[keywords.food]
terms = ["food", "eat", "pizza", "burger", "coffee", "cake", "dessert", "snack"]

[keywords.classic]
terms = ["classic", "text", "ascii", "emoticon", "smiley", "shrug", "tableflip"]
"#;

/// Settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Maximum number of recently used emoticons to keep
    #[serde(default = "default_max_recent")]
    pub max_recent: usize,

    /// Keywords for each emoticon category
    #[serde(default)]
    pub keywords: HashMap<String, CategoryKeywords>,
}

fn default_max_recent() -> usize {
    10
}

/// Keywords for a category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryKeywords {
    pub terms: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        // Parse default settings from TOML
        toml::from_str(DEFAULT_SETTINGS_TOML).unwrap_or_else(|_| Settings {
            max_recent: 10,
            keywords: HashMap::new(),
        })
    }
}

/// Recent emoticons structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecentEmoticons {
    pub emoticons: Vec<String>,
}

/// Configuration manager
pub struct Config {
    #[allow(dead_code)]
    config_dir: PathBuf,
    #[allow(dead_code)]
    settings_file: PathBuf,
    recent_file: PathBuf,
    settings: Settings,
}

impl Config {
    /// Create a new configuration manager
    pub fn new() -> std::io::Result<Self> {
        // Use ~/.smile directory
        let home_dir = dirs::home_dir().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find home directory")
        })?;

        let config_dir = home_dir.join(".smile");
        let settings_file = config_dir.join("settings.toml");
        let recent_file = config_dir.join("recent.json");

        // Create directory if it doesn't exist
        fs::create_dir_all(&config_dir)?;

        // Initialize settings file if it doesn't exist
        if !settings_file.exists() {
            fs::write(&settings_file, DEFAULT_SETTINGS_TOML)?;
        }

        // Load settings
        let settings = Self::load_settings(&settings_file)?;

        Ok(Config {
            config_dir,
            settings_file,
            recent_file,
            settings,
        })
    }

    /// Load settings from file
    fn load_settings(settings_file: &PathBuf) -> std::io::Result<Settings> {
        let content = fs::read_to_string(settings_file)?;
        toml::from_str(&content).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse settings.toml: {}", e),
            )
        })
    }

    /// Get settings
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Reload settings from file
    #[allow(dead_code)]
    pub fn reload_settings(&mut self) -> std::io::Result<()> {
        self.settings = Self::load_settings(&self.settings_file)?;
        Ok(())
    }

    /// Load recently used emoticons
    pub fn load_recent(&self) -> Vec<String> {
        if let Ok(content) = fs::read_to_string(&self.recent_file) {
            if let Ok(recent) = serde_json::from_str::<RecentEmoticons>(&content) {
                return recent.emoticons;
            }
        }

        // Try to migrate from old location
        self.migrate_from_old_config()
    }

    /// Save recently used emoticons
    pub fn save_recent(&self, emoticons: &[String]) -> std::io::Result<()> {
        let recent = RecentEmoticons {
            emoticons: emoticons.to_vec(),
        };
        let json = serde_json::to_string_pretty(&recent)?;
        fs::write(&self.recent_file, json)?;
        Ok(())
    }

    /// Migrate from old config location (~/.config/smile/history.json)
    fn migrate_from_old_config(&self) -> Vec<String> {
        if let Some(config_dir) = dirs::config_dir() {
            let old_file = config_dir.join("smile").join("history.json");
            if old_file.exists() {
                if let Ok(content) = fs::read_to_string(&old_file) {
                    #[derive(Deserialize)]
                    struct OldConfig {
                        history: Vec<String>,
                    }
                    if let Ok(old_config) = serde_json::from_str::<OldConfig>(&content) {
                        // Save to new location
                        let _ = self.save_recent(&old_config.history);
                        return old_config.history;
                    }
                }
            }
        }
        Vec::new()
    }

    /// Check if a search query matches any keywords for a category
    pub fn matches_category_keywords(&self, category: &str, query: &str) -> bool {
        let query_lower = query.to_lowercase();

        // Check category name itself
        if category.to_lowercase().contains(&query_lower) {
            return true;
        }

        // Check keywords
        if let Some(keywords) = self.settings.keywords.get(&category.to_lowercase()) {
            for term in &keywords.terms {
                if term.to_lowercase().contains(&query_lower) {
                    return true;
                }
            }
        }

        false
    }

    /// Get configuration directory path
    #[allow(dead_code)]
    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.max_recent, 10);
        assert!(settings.keywords.len() > 0);
    }

    #[test]
    fn test_settings_toml_parse() {
        let toml_content = r#"
max_recent = 15

[keywords]

[keywords.happy]
terms = ["happy", "joy"]
        "#;

        let settings: Settings = toml::from_str(toml_content).unwrap();
        assert_eq!(settings.max_recent, 15);
        assert!(settings.keywords.contains_key("happy"));
    }
}

