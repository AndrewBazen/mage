use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct MageConfig {
    pub shell: Option<String>,
    pub options: HashMap<String, String>,
}

impl MageConfig {
    pub fn load_from_file(path: &Path) -> Self {
        let mut config = MageConfig::default();

        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();

                    if key == "shell" {
                        config.shell = Some(value.to_string());
                    } else {
                        config.options.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        config
    }

    pub fn find_config() -> Option<Self> {
        // Check current directory
        let current_config = Path::new(".mageconfig");
        if current_config.exists() {
            return Some(Self::load_from_file(current_config));
        }

        // Check parent directories
        let mut current_dir = std::env::current_dir().ok()?;
        while let Some(parent) = current_dir.parent() {
            let config_path = parent.join(".mageconfig");
            if config_path.exists() {
                return Some(Self::load_from_file(&config_path));
            }
            current_dir = parent.to_path_buf();
        }

        // Check home directory
        if let Some(home) = dirs_next::home_dir() {
            let home_config = home.join(".mageconfig");
            if home_config.exists() {
                return Some(Self::load_from_file(&home_config));
            }
        }

        None
    }
}
