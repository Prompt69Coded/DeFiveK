use serde::{Deserialize, Serialize};

/// Shell theme mode selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        matches!(self, ThemeMode::Dark)
    }

    pub fn toggle(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Dark
    }
}

impl From<bool> for ThemeMode {
    fn from(dark: bool) -> Self {
        if dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        }
    }
}

impl From<ThemeMode> for bool {
    fn from(mode: ThemeMode) -> Self {
        mode.is_dark()
    }
}
