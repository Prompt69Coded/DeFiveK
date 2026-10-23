use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

use crate::modes::DesktopMode;

const QUALIFIER: &str = "org";
const ORGANIZATION: &str = "defivek";
const APPLICATION: &str = "defivek";
const CONFIG_FILE_NAME: &str = "config.toml";

fn default_wallpaper() -> String {
    "aurora-borealis".to_string()
}

/// Configuration settings for DeFiveK Desktop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiveKConfig {
    /// Active desktop mode (Windows, MacOS, Android, Aurora).
    pub mode: DesktopMode,
    /// Whether dark theme is enabled.
    pub dark_mode: bool,
    /// Whether the top floating shell mode-switch control bar is visible.
    pub show_shell_controls: bool,
    /// Default window width in logical pixels.
    pub window_width: u32,
    /// Default window height in logical pixels.
    pub window_height: u32,
    /// Active desktop wallpaper ID.
    #[serde(default = "default_wallpaper")]
    pub wallpaper: String,
}

impl Default for DeFiveKConfig {
    fn default() -> Self {
        Self {
            mode: DesktopMode::Windows,
            dark_mode: true,
            show_shell_controls: true,
            window_width: 1200,
            window_height: 760,
            wallpaper: default_wallpaper(),
        }
    }
}

impl DeFiveKConfig {
    /// Determines the config file path according to XDG/OS conventions or environment overrides.
    pub fn config_path() -> PathBuf {
        if let Ok(custom_path) = std::env::var("DEFIVEK_CONFIG_PATH") {
            return PathBuf::from(custom_path);
        }

        if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            proj_dirs.config_dir().join(CONFIG_FILE_NAME)
        } else {
            // Fallback to relative local directory if user directories cannot be determined
            PathBuf::from(".config").join(CONFIG_FILE_NAME)
        }
    }

    /// Loads the configuration from disk, falling back gracefully to defaults if absent or malformed.
    pub fn load() -> Self {
        let path = Self::config_path();
        if !path.exists() {
            eprintln!("[DeFiveK Shell] Notice: No existing config at {:?}. Using defaults.", path);
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str::<DeFiveKConfig>(&content) {
                Ok(config) => {
                    eprintln!("[DeFiveK Shell] Loaded config from {:?}", path);
                    config
                }
                Err(err) => {
                    eprintln!(
                        "[DeFiveK Shell] Warning: Failed to parse config file ({:?}): {}. Using defaults.",
                        path, err
                    );
                    Self::default()
                }
            },
            Err(err) => {
                eprintln!(
                    "[DeFiveK Shell] Warning: Failed to read config file ({:?}): {}. Using defaults.",
                    path, err
                );
                Self::default()
            }
        }
    }

    /// Saves the current configuration to disk.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(err) = fs::create_dir_all(parent) {
                    return Err(format!(
                        "Failed to create config directory {:?}: {}",
                        parent, err
                    ));
                }
            }
        }

        let serialized = match toml::to_string_pretty(self) {
            Ok(s) => s,
            Err(err) => return Err(format!("Failed to serialize config to TOML: {}", err)),
        };

        if let Err(err) = fs::write(&path, serialized) {
            return Err(format!("Failed to write config file {:?}: {}", path, err));
        }

        eprintln!("[DeFiveK Shell] Saved updated configuration to {:?}", path);
        Ok(())
    }
}
