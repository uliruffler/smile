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

[keywords.laughing]
terms = ["laugh", "lol", "lmao", "rofl", "haha", "hehe", "funny", "hilarious", "tears", "crying", "joy"]

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

# Keywords for individual emoticons
# Format: [emoticon_keywords."emoji"] with terms = ["keyword1", "keyword2", ...]
[emoticon_keywords]

[emoticon_keywords."😀"]
terms = ["grinning", "smile", "happy"]

[emoticon_keywords."😃"]
terms = ["smiley", "smile", "happy", "joy"]

[emoticon_keywords."😄"]
terms = ["smile", "happy", "joy", "laugh", "pleased"]

[emoticon_keywords."😁"]
terms = ["grin", "smile", "happy", "teeth"]

[emoticon_keywords."😆"]
terms = ["laugh", "satisfied", "happy", "giggle", "haha"]

[emoticon_keywords."😊"]
terms = ["blush", "smile", "happy", "pleased", "shy"]

[emoticon_keywords."😇"]
terms = ["angel", "innocent", "halo", "saint"]

[emoticon_keywords."🙂"]
terms = ["smile", "happy", "simple"]

[emoticon_keywords."🙃"]
terms = ["upside", "down", "silly", "sarcasm"]

[emoticon_keywords."😉"]
terms = ["wink", "flirt", "playful"]

[emoticon_keywords."😌"]
terms = ["relieved", "content", "peaceful", "calm"]

[emoticon_keywords."😍"]
terms = ["love", "heart", "eyes", "adore", "crush"]

[emoticon_keywords."🥰"]
terms = ["love", "hearts", "smile", "adore", "affection"]

[emoticon_keywords."😘"]
terms = ["kiss", "blow", "love", "mwah"]

[emoticon_keywords."😂"]
terms = ["laugh", "tears", "joy", "lol", "crying", "funny", "haha"]

[emoticon_keywords."🤣"]
terms = ["rofl", "laugh", "rolling", "floor", "hilarious", "lmao", "funny"]

[emoticon_keywords."😹"]
terms = ["cat", "laugh", "tears", "joy", "lol", "funny"]

[emoticon_keywords."😅"]
terms = ["grin", "sweat", "nervous", "laugh", "relief", "phew"]

[emoticon_keywords."🥲"]
terms = ["smile", "tear", "touched", "grateful", "bittersweet", "emotional"]

[emoticon_keywords."☺️"]
terms = ["smile", "blush", "happy", "pleased", "content"]

[emoticon_keywords."😢"]
terms = ["cry", "tear", "sad", "upset"]

[emoticon_keywords."😭"]
terms = ["sob", "cry", "tears", "bawl", "weep"]

[emoticon_keywords."😿"]
terms = ["cat", "cry", "sad", "tears"]

[emoticon_keywords."😔"]
terms = ["pensive", "sad", "dejected", "sorry"]

[emoticon_keywords."😞"]
terms = ["disappointed", "sad", "upset"]

[emoticon_keywords."😟"]
terms = ["worried", "concerned", "sad"]

[emoticon_keywords."😥"]
terms = ["sad", "sweat", "disappointed", "relief"]

[emoticon_keywords."😰"]
terms = ["anxious", "nervous", "sweat", "worried"]

[emoticon_keywords."😨"]
terms = ["fearful", "scared", "worried", "afraid"]

[emoticon_keywords."😧"]
terms = ["anguished", "stunned", "shocked"]

[emoticon_keywords."😦"]
terms = ["frowning", "sad", "upset", "worried"]

[emoticon_keywords."😠"]
terms = ["angry", "mad", "upset", "annoyed"]

[emoticon_keywords."😡"]
terms = ["rage", "angry", "mad", "furious", "pissed"]

[emoticon_keywords."🤬"]
terms = ["cursing", "swear", "angry", "symbols", "mad"]

[emoticon_keywords."😤"]
terms = ["triumph", "smug", "proud", "frustrated"]

[emoticon_keywords."😾"]
terms = ["cat", "pouting", "angry", "grumpy"]

[emoticon_keywords."💢"]
terms = ["anger", "symbol", "mad", "comic"]

[emoticon_keywords."😮"]
terms = ["wow", "surprised", "open", "mouth"]

[emoticon_keywords."😯"]
terms = ["hushed", "surprised", "shocked", "quiet"]

[emoticon_keywords."😲"]
terms = ["astonished", "shocked", "amazed", "gasp"]

[emoticon_keywords."😳"]
terms = ["flushed", "embarrassed", "shy", "surprised"]

[emoticon_keywords."🤯"]
terms = ["mind", "blown", "exploding", "shocked", "amazed"]

[emoticon_keywords."❤️"]
terms = ["heart", "love", "red"]

[emoticon_keywords."💕"]
terms = ["hearts", "two", "love", "pink"]

[emoticon_keywords."💖"]
terms = ["sparkling", "heart", "love", "sparkle"]

[emoticon_keywords."💗"]
terms = ["growing", "heart", "love", "heartbeat"]

[emoticon_keywords."💓"]
terms = ["beating", "heart", "love", "heartbeat"]

[emoticon_keywords."💞"]
terms = ["revolving", "hearts", "love"]

[emoticon_keywords."💝"]
terms = ["gift", "heart", "love", "present"]

[emoticon_keywords."💘"]
terms = ["cupid", "arrow", "heart", "love"]

[emoticon_keywords."💟"]
terms = ["heart", "decoration", "love"]

[emoticon_keywords."♥️"]
terms = ["heart", "suit", "love", "card"]

[emoticon_keywords."👍"]
terms = ["thumbs", "up", "yes", "ok", "good", "like", "agree"]

[emoticon_keywords."👎"]
terms = ["thumbs", "down", "no", "bad", "dislike", "disagree"]

[emoticon_keywords."👌"]
terms = ["ok", "okay", "hand", "perfect", "good"]

[emoticon_keywords."✌️"]
terms = ["peace", "victory", "two"]

[emoticon_keywords."🤞"]
terms = ["fingers", "crossed", "luck", "hope", "wish"]

[emoticon_keywords."🤘"]
terms = ["rock", "on", "horns", "metal", "devil"]

[emoticon_keywords."🤙"]
terms = ["call", "me", "hang", "loose", "shaka"]

[emoticon_keywords."👏"]
terms = ["clap", "applause", "praise", "bravo"]

[emoticon_keywords."🙌"]
terms = ["raising", "hands", "celebrate", "yay", "hooray"]

[emoticon_keywords."👐"]
terms = ["open", "hands", "hug"]

[emoticon_keywords."🤲"]
terms = ["palms", "together", "pray", "please"]

[emoticon_keywords."🤝"]
terms = ["handshake", "deal", "agreement", "shake"]

[emoticon_keywords."🙏"]
terms = ["pray", "please", "thanks", "hope", "namaste"]

[emoticon_keywords."😐"]
terms = ["neutral", "meh", "blank"]

[emoticon_keywords."😑"]
terms = ["expressionless", "blank", "annoyed", "meh"]

[emoticon_keywords."😶"]
terms = ["no", "mouth", "quiet", "silent"]

[emoticon_keywords."🙄"]
terms = ["eye", "roll", "annoyed", "whatever"]

[emoticon_keywords."😏"]
terms = ["smirk", "smug", "sly"]

[emoticon_keywords."😣"]
terms = ["persevere", "struggle", "effort"]

[emoticon_keywords."😪"]
terms = ["sleepy", "tired", "sleep"]

[emoticon_keywords."😫"]
terms = ["tired", "exhausted", "weary"]

[emoticon_keywords."🥱"]
terms = ["yawn", "tired", "bored", "sleepy"]

[emoticon_keywords."😴"]
terms = ["sleeping", "zzz", "sleep", "tired"]

[emoticon_keywords."😎"]
terms = ["cool", "sunglasses", "awesome"]

[emoticon_keywords."🤓"]
terms = ["nerd", "geek", "glasses", "smart"]

[emoticon_keywords."🧐"]
terms = ["monocle", "face", "thinking", "fancy"]

[emoticon_keywords."😺"]
terms = ["cat", "grinning", "happy", "smile"]

[emoticon_keywords."😸"]
terms = ["cat", "grin", "smile", "happy"]

[emoticon_keywords."😹"]
terms = ["cat", "joy", "tears", "laugh"]

[emoticon_keywords."😻"]
terms = ["cat", "heart", "eyes", "love"]

[emoticon_keywords."😼"]
terms = ["cat", "smirk", "sly"]

[emoticon_keywords."😽"]
terms = ["cat", "kiss", "love"]

[emoticon_keywords."🙀"]
terms = ["cat", "weary", "surprised", "shocked"]

[emoticon_keywords."⭐"]
terms = ["star", "favorite", "fave"]

[emoticon_keywords."✨"]
terms = ["sparkles", "shine", "magic", "glitter"]

[emoticon_keywords."🌟"]
terms = ["glowing", "star", "shine"]

[emoticon_keywords."💫"]
terms = ["dizzy", "star", "sparkle"]

[emoticon_keywords."🔥"]
terms = ["fire", "hot", "flame", "lit"]

[emoticon_keywords."💥"]
terms = ["boom", "explosion", "bang", "collision"]

[emoticon_keywords."💦"]
terms = ["sweat", "droplets", "water", "splash"]

[emoticon_keywords."💨"]
terms = ["dash", "wind", "fast", "smoke"]

[emoticon_keywords."✅"]
terms = ["check", "mark", "yes", "done", "correct"]

[emoticon_keywords."❌"]
terms = ["cross", "mark", "no", "wrong", "x"]

[emoticon_keywords."⚡"]
terms = ["lightning", "bolt", "electric", "fast", "zap"]

[emoticon_keywords."🌈"]
terms = ["rainbow", "pride", "colors"]

[emoticon_keywords."🎉"]
terms = ["party", "popper", "celebrate", "confetti"]

[emoticon_keywords."🎊"]
terms = ["confetti", "ball", "party", "celebrate"]

[emoticon_keywords."🎈"]
terms = ["balloon", "party", "celebrate"]

[emoticon_keywords."🎁"]
terms = ["gift", "present", "wrapped", "birthday"]

[emoticon_keywords."🏆"]
terms = ["trophy", "win", "award", "champion"]

[emoticon_keywords."🥇"]
terms = ["first", "place", "medal", "gold", "win"]

[emoticon_keywords."🥈"]
terms = ["second", "place", "medal", "silver"]

[emoticon_keywords."🥉"]
terms = ["third", "place", "medal", "bronze"]

[emoticon_keywords."🏅"]
terms = ["medal", "sports", "award"]

[emoticon_keywords."🎖️"]
terms = ["military", "medal", "honor"]

[emoticon_keywords."🐶"]
terms = ["dog", "puppy", "pet"]

[emoticon_keywords."🐱"]
terms = ["cat", "kitty", "pet"]

[emoticon_keywords."🐭"]
terms = ["mouse", "pet"]

[emoticon_keywords."🐹"]
terms = ["hamster", "pet"]

[emoticon_keywords."🐰"]
terms = ["rabbit", "bunny", "pet"]

[emoticon_keywords."🦊"]
terms = ["fox", "sly"]

[emoticon_keywords."🐻"]
terms = ["bear"]

[emoticon_keywords."🐼"]
terms = ["panda", "bear"]

[emoticon_keywords."🐨"]
terms = ["koala", "bear"]

[emoticon_keywords."🐯"]
terms = ["tiger", "face"]

[emoticon_keywords."🦁"]
terms = ["lion", "face"]

[emoticon_keywords."🐮"]
terms = ["cow", "face"]

[emoticon_keywords."🐷"]
terms = ["pig", "face"]

[emoticon_keywords."🐸"]
terms = ["frog", "face"]

[emoticon_keywords."🐵"]
terms = ["monkey", "face"]

[emoticon_keywords."🍕"]
terms = ["pizza", "slice", "food"]

[emoticon_keywords."🍔"]
terms = ["burger", "hamburger", "food"]

[emoticon_keywords."🍟"]
terms = ["fries", "french", "food"]

[emoticon_keywords."🌭"]
terms = ["hotdog", "hot", "dog", "food"]

[emoticon_keywords."🍿"]
terms = ["popcorn", "movie", "snack"]

[emoticon_keywords."🧂"]
terms = ["salt", "shaker", "seasoning"]

[emoticon_keywords."🍰"]
terms = ["cake", "shortcake", "dessert"]

[emoticon_keywords."🎂"]
terms = ["birthday", "cake", "dessert", "celebrate"]

[emoticon_keywords."🍩"]
terms = ["donut", "doughnut", "dessert"]

[emoticon_keywords."🍪"]
terms = ["cookie", "dessert", "snack"]

[emoticon_keywords."🍫"]
terms = ["chocolate", "bar", "dessert"]

[emoticon_keywords."🍬"]
terms = ["candy", "sweet"]

[emoticon_keywords."🍭"]
terms = ["lollipop", "candy", "sweet"]

[emoticon_keywords."☕"]
terms = ["coffee", "hot", "beverage", "drink"]

[emoticon_keywords."🍵"]
terms = ["tea", "cup", "hot", "beverage"]

[emoticon_keywords.":-)"]
terms = ["smile", "happy", "classic"]

[emoticon_keywords.":)"]
terms = ["smile", "happy", "classic"]

[emoticon_keywords.":("]
terms = ["sad", "frown", "classic"]

[emoticon_keywords.":-("]
terms = ["sad", "frown", "classic"]

[emoticon_keywords.";-)"]
terms = ["wink", "classic"]

[emoticon_keywords.";)"]
terms = ["wink", "classic"]

[emoticon_keywords.":-D"]
terms = ["big", "smile", "laugh", "classic"]

[emoticon_keywords.":D"]
terms = ["big", "smile", "laugh", "classic"]

[emoticon_keywords.":-P"]
terms = ["tongue", "playful", "classic"]

[emoticon_keywords.":P"]
terms = ["tongue", "playful", "classic"]

[emoticon_keywords.":-O"]
terms = ["surprised", "shocked", "classic"]

[emoticon_keywords.":O"]
terms = ["surprised", "shocked", "classic"]

[emoticon_keywords.":-|"]
terms = ["neutral", "meh", "classic"]

[emoticon_keywords.":|"]
terms = ["neutral", "meh", "classic"]

[emoticon_keywords."<3"]
terms = ["heart", "love", "classic"]

[emoticon_keywords."</3"]
terms = ["broken", "heart", "sad", "classic"]

[emoticon_keywords.":*"]
terms = ["kiss", "love", "classic"]

[emoticon_keywords.":-*"]
terms = ["kiss", "love", "classic"]

[emoticon_keywords."^_^"]
terms = ["happy", "anime", "classic"]

[emoticon_keywords."^.^"]
terms = ["happy", "cute", "classic"]

[emoticon_keywords."o_o"]
terms = ["surprised", "shocked", "classic"]

[emoticon_keywords."O_O"]
terms = ["shocked", "wide", "eyes", "classic"]

[emoticon_keywords."T_T"]
terms = ["crying", "tears", "sad", "classic"]

[emoticon_keywords."ToT"]
terms = ["crying", "tears", "sad", "classic"]

[emoticon_keywords.">_<"]
terms = ["frustrated", "annoyed", "classic"]

[emoticon_keywords."-_-"]
terms = ["annoyed", "tired", "classic"]

[emoticon_keywords."¯\\_(ツ)_/¯"]
terms = ["shrug", "whatever", "idk", "dunno", "classic"]

[emoticon_keywords."(╯°□°）╯︵ ┻━┻"]
terms = ["tableflip", "rage", "angry", "flip", "table", "classic"]

[emoticon_keywords."(ಠ_ಠ)"]
terms = ["disapproval", "look", "judgement", "classic"]

[emoticon_keywords."(◕‿◕)"]
terms = ["happy", "cute", "classic"]

[emoticon_keywords."(づ｡◕‿‿◕｡)づ"]
terms = ["hug", "love", "cuddle", "classic"]

[emoticon_keywords."ʕ•ᴥ•ʔ"]
terms = ["bear", "cute", "classic"]
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

    /// Keywords for individual emoticons
    #[serde(default)]
    pub emoticon_keywords: HashMap<String, EmoticonKeywords>,
}

/// Window state structure for remembering window dimensions and position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

fn default_max_recent() -> usize {
    10
}

/// Keywords for a category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryKeywords {
    pub terms: Vec<String>,
}

/// Keywords for an individual emoticon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmoticonKeywords {
    pub terms: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        // Parse default settings from TOML
        toml::from_str(DEFAULT_SETTINGS_TOML).unwrap_or_else(|_| Settings {
            max_recent: 10,
            keywords: HashMap::new(),
            emoticon_keywords: HashMap::new(),
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
    window_state_file: PathBuf,
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
        let window_state_file = config_dir.join("window_state.json");

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
            window_state_file,
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

    /// Check if a search query matches any keywords for an emoticon
    pub fn matches_emoticon_keywords(&self, emoticon: &str, query: &str) -> bool {
        let query_lower = query.to_lowercase();

        // Check the emoticon itself
        if emoticon.to_lowercase().contains(&query_lower) {
            return true;
        }

        // Check keywords for this specific emoticon
        if let Some(keywords) = self.settings.emoticon_keywords.get(emoticon) {
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

    /// Load window state
    pub fn load_window_state(&self) -> Option<WindowState> {
        if let Ok(content) = fs::read_to_string(&self.window_state_file) {
            if let Ok(state) = serde_json::from_str::<WindowState>(&content) {
                return Some(state);
            }
        }
        None
    }

    /// Save window state
    pub fn save_window_state(&self, state: &WindowState) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(state)?;
        fs::write(&self.window_state_file, json)?;
        Ok(())
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

    #[test]
    fn test_emoticon_keywords() {
        let settings = Settings::default();
        // Check that emoticon keywords are loaded
        assert!(settings.emoticon_keywords.len() > 0);

        // Check a specific emoticon has keywords
        if let Some(fire_keywords) = settings.emoticon_keywords.get("🔥") {
            assert!(fire_keywords.terms.contains(&"fire".to_string()));
        }
    }

    #[test]
    fn test_emoticon_keyword_matching() {
        // Create a minimal config for testing
        let settings = Settings::default();
        let config_dir = std::env::temp_dir().join("smile_test");
        let settings_file = config_dir.join("settings.toml");
        let recent_file = config_dir.join("recent.json");

        let config = Config {
            config_dir,
            settings_file,
            recent_file,
            settings,
        };

        // Test that fire emoji matches "fire" keyword
        assert!(config.matches_emoticon_keywords("🔥", "fire"));

        // Test that thumbs up matches "thumbs" keyword
        assert!(config.matches_emoticon_keywords("👍", "thumbs"));

        // Test that pizza matches "pizza" keyword
        assert!(config.matches_emoticon_keywords("🍕", "pizza"));

        // Test case insensitivity
        assert!(config.matches_emoticon_keywords("🔥", "FIRE"));

        // Test partial matching
        assert!(config.matches_emoticon_keywords("🔥", "fir"));
    }
}
