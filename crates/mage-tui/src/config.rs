//! Configuration for the Mage TUI shell
//!
//! Handles loading and saving user preferences for:
//! - Layout settings
//! - Theme/colors
//! - Keybindings
//! - Panel visibility defaults

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub layout: LayoutConfig,
    pub theme: String,
    pub keybinds: KeybindConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub style: LayoutStyle,
    pub output_width: u16,
    pub context_width: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutStyle {
    Split,
    Minimal,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindConfig {
    pub command_palette: String,
    pub file_browser: String,
    pub git_panel: String,
    pub quit: String,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            layout: LayoutConfig {
                style: LayoutStyle::Split,
                output_width: 60,
                context_width: 40,
            },
            theme: "default".to_string(),
            keybinds: KeybindConfig {
                command_palette: "ctrl+p".to_string(),
                file_browser: "ctrl+f".to_string(),
                git_panel: "ctrl+g".to_string(),
                quit: "ctrl+c".to_string(),
            },
        }
    }
}

impl TuiConfig {
    pub fn load() -> Self {
        // Try to load from ~/.config/mage/tui.toml
        let config_path = dirs::config_dir().map(|p| p.join("mage").join("tui.toml"));

        if let Some(path) = config_path
            && let Ok(content) = std::fs::read_to_string(&path)
            && let Ok(config) = toml::from_str(&content)
        {
            return config;
        }

        Self::default()
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), String> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("mage");

        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;

        let config_path = config_dir.join("tui.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        std::fs::write(config_path, content).map_err(|e| format!("Failed to write config: {}", e))
    }
}
