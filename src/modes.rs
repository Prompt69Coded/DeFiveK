use serde::{Deserialize, Serialize};
use std::fmt;

/// Desktop interaction and layout modes available in DeFiveK Desktop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DesktopMode {
    /// Windows 7-inspired layout: Bottom taskbar, Start launcher panel, system tray, desktop icons.
    Windows = 0,
    /// OS X Yosemite-inspired layout: Top menu bar, bottom application dock, floating windows.
    MacOS = 1,
    /// Android 10-inspired layout: Top status bar, widget home screen, app drawer grid, quick settings.
    Android = 2,
    /// Original lightweight layout: Compact status bar, tiling panes, keyboard-first workflow.
    Aurora = 3,
}

impl DesktopMode {
    /// Returns the integer ID corresponding to the mode (used in Slint UI bindings).
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    /// Converts an integer ID into a DesktopMode.
    pub fn from_i32(val: i32) -> Self {
        match val {
            0 => DesktopMode::Windows,
            1 => DesktopMode::MacOS,
            2 => DesktopMode::Android,
            3 => DesktopMode::Aurora,
            _ => DesktopMode::Windows,
        }
    }

    /// User-friendly display name.
    pub fn title(&self) -> &'static str {
        match self {
            DesktopMode::Windows => "Windows Mode",
            DesktopMode::MacOS => "macOS Mode",
            DesktopMode::Android => "Android Mode",
            DesktopMode::Aurora => "Aurora Mode",
        }
    }

    /// Subtitle describing inspiration / paradigm.
    pub fn subtitle(&self) -> &'static str {
        match self {
            DesktopMode::Windows => "Modernized Windows 7 desktop workflow",
            DesktopMode::MacOS => "Refined OS X Yosemite menu bar & dock",
            DesktopMode::Android => "Desktop-friendly Android 10 home & app grid",
            DesktopMode::Aurora => "Original lightweight keyboard-first tiling shell",
        }
    }

    /// Returns a list of all available desktop modes.
    pub fn all() -> &'static [DesktopMode] {
        &[
            DesktopMode::Windows,
            DesktopMode::MacOS,
            DesktopMode::Android,
            DesktopMode::Aurora,
        ]
    }
}

impl Default for DesktopMode {
    fn default() -> Self {
        DesktopMode::Windows
    }
}

impl fmt::Display for DesktopMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title())
    }
}

impl From<i32> for DesktopMode {
    fn from(val: i32) -> Self {
        Self::from_i32(val)
    }
}

impl From<DesktopMode> for i32 {
    fn from(mode: DesktopMode) -> Self {
        mode.as_i32()
    }
}
