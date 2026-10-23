use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;
use chrono::Local;
use slint::{ComponentHandle, ModelRc, SharedString, Timer, TimerMode, VecModel};

use crate::config::DeFiveKConfig;
use crate::modes::DesktopMode;

// Include Slint generated bindings
slint::include_modules!();

/// Coordinates runtime state, configuration persistence, and UI events for DeFiveK Desktop.
#[allow(dead_code)]
pub struct DeFiveKApp {
    config: Rc<RefCell<DeFiveKConfig>>,
    window: MainWindow,
    _clock_timer: Timer,
    history_stack: Rc<RefCell<Vec<PathBuf>>>,
}

impl DeFiveKApp {
    /// Formats a byte size into human-readable representation.
    fn format_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;

        if bytes >= GB {
            format!("{:.1} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    /// Discovers an available external terminal emulator.
    fn discover_host_terminal() -> (&'static str, &'static str) {
        if cfg!(target_os = "macos") {
            return ("Launch Terminal.app", "open -a Terminal");
        }

        let candidates = [
            ("x-terminal-emulator", "x-terminal-emulator"),
            ("kitty", "kitty"),
            ("alacritty", "alacritty"),
            ("gnome-terminal", "gnome-terminal"),
            ("xfce4-terminal", "xfce4-terminal"),
            ("konsole", "konsole"),
            ("xterm", "xterm"),
        ];

        for (name, cmd) in candidates {
            if let Ok(output) = Command::new("which").arg(cmd).output() {
                if output.status.success() {
                    return (name, cmd);
                }
            }
        }

        ("Host Terminal (Default)", "x-terminal-emulator")
    }

    /// Safely resolves a path string with tilde or relative expansion.
    fn resolve_path(p: &str) -> PathBuf {
        if p == "~" || p.starts_with("~/") {
            let home = env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/"));
            if p == "~" {
                home
            } else {
                home.join(&p[2..])
            }
        } else if p == "." {
            env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
        } else {
            PathBuf::from(p)
        }
    }

    /// Safely scans a directory and converts its entries into Slint FileItemData models.
    fn read_directory(dir_path: &Path) -> Result<Vec<FileItemData>, String> {
        let read_dir = fs::read_dir(dir_path)
            .map_err(|e| format!("Could not read directory {:?}: {}", dir_path, e))?;

        let mut entries = Vec::new();
        for entry_res in read_dir {
            if let Ok(entry) = entry_res {
                let path = entry.path();
                let file_name = entry.file_name().to_string_lossy().to_string();

                // Skip hidden files starting with '.' unless needed
                if file_name.starts_with('.') && file_name != ".config" {
                    continue;
                }

                let is_dir = path.is_dir();
                let (icon, file_type, size_str) = if is_dir {
                    ("📁", "Folder", "<dir>".to_string())
                } else {
                    let ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();

                    let (ic, ft) = match ext.as_str() {
                        "rs" => ("🦀", "Rust Source"),
                        "slint" => ("🎨", "Slint GUI"),
                        "toml" => ("⚙️", "Config TOML"),
                        "json" => ("⚙️", "JSON Data"),
                        "md" => ("📄", "Markdown"),
                        "txt" => ("📄", "Plain Text"),
                        "sh" => ("⚡", "Shell Script"),
                        "png" | "jpg" | "jpeg" | "svg" => ("🖼️", "Image"),
                        "lock" => ("🔒", "Lockfile"),
                        _ => ("📄", "File"),
                    };

                    let size = entry
                        .metadata()
                        .map(|m| Self::format_size(m.len()))
                        .unwrap_or_else(|_| "0 B".to_string());

                    (ic, ft, size)
                };

                entries.push((
                    is_dir,
                    file_name.to_lowercase(),
                    FileItemData {
                        name: SharedString::from(file_name),
                        path: SharedString::from(path.to_string_lossy().to_string()),
                        is_dir,
                        size_str: SharedString::from(size_str),
                        icon: SharedString::from(icon),
                        file_type: SharedString::from(file_type),
                    },
                ));
            }
        }

        // Sort: directories first, then alphabetical by name
        entries.sort_by(|a, b| match (a.0, b.0) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.1.cmp(&b.1),
        });

        Ok(entries.into_iter().map(|e| e.2).collect())
    }

    /// Initializes the DeFiveK Desktop application with the specified configuration.
    pub fn new(initial_config: DeFiveKConfig, fullscreen: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let window = MainWindow::new()?;
        let config_rc = Rc::new(RefCell::new(initial_config));
        let history_stack = Rc::new(RefCell::new(Vec::<PathBuf>::new()));

        if fullscreen {
            window.window().set_fullscreen(true);
        }

        // Apply initial configuration state to UI properties
        let cfg = config_rc.borrow();
        window.set_current_mode(cfg.mode.as_i32());
        window.set_dark_mode(cfg.dark_mode);
        window.set_show_shell_controls(cfg.show_shell_controls);
        window.set_selected_wallpaper(SharedString::from(cfg.wallpaper.clone()));

        // Synchronize ShellPalette Slint global
        let palette = window.global::<ShellPalette>();
        palette.set_dark_mode(cfg.dark_mode);

        let now = Local::now();
        window.set_clock_time(SharedString::from(now.format("%I:%M %p").to_string()));
        window.set_clock_date(SharedString::from(now.format("%A, %b %e").to_string()));
        window.set_status_banner(SharedString::from(format!(
            "DeFiveK Desktop active in {}",
            cfg.mode.title()
        )));
        window.set_status_detail(SharedString::from(format!(
            "{} | Debian 13 ARM64",
            cfg.mode.subtitle()
        )));
        drop(cfg);

        // Populate initial File Explorer state
        let initial_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        window.set_files_current_path(SharedString::from(initial_dir.to_string_lossy().to_string()));
        match Self::read_directory(&initial_dir) {
            Ok(items) => {
                let count = items.len();
                let model = Rc::new(VecModel::from(items));
                window.set_files_items(ModelRc::from(model));
                window.set_files_has_error(false);
                window.set_files_status_text(SharedString::from(format!("{} items in folder", count)));
            }
            Err(e) => {
                window.set_files_has_error(true);
                window.set_files_error_text(SharedString::from(e));
            }
        }

        // Host terminal discovery
        let (term_label, _) = Self::discover_host_terminal();
        window.set_host_terminal_name(SharedString::from(term_label));

        // Initial Text Editor file (README.md)
        let readme_path = initial_dir.join("README.md");
        if let Ok(content) = fs::read_to_string(&readme_path) {
            window.set_edit_file_path(SharedString::from(readme_path.to_string_lossy().to_string()));
            window.set_edit_file_content(SharedString::from(content));
            window.set_edit_status(SharedString::from("README.md loaded ready for editing"));
        }

        // ==========================================
        // Event Handlers & Callbacks
        // ==========================================

        // 1. Mode Change Callback
        {
            let config_clone = Rc::clone(&config_rc);
            let window_weak = window.as_weak();
            window.on_mode_changed(move |mode_idx| {
                let new_mode = DesktopMode::from_i32(mode_idx);
                eprintln!("[DeFiveK Shell] Mode switched to: {:?}", new_mode);

                let mut cfg = config_clone.borrow_mut();
                cfg.mode = new_mode;
                if let Err(e) = cfg.save() {
                    eprintln!("[DeFiveK Shell] Warning saving config: {}", e);
                }

                if let Some(win) = window_weak.upgrade() {
                    win.set_status_banner(SharedString::from(format!(
                        "Switched to {}",
                        new_mode.title()
                    )));
                    win.set_status_detail(SharedString::from(format!(
                        "{} | Saved to config",
                        new_mode.subtitle()
                    )));
                }
            });
        }

        // 2. Theme Toggle Callback (Reactively updates ShellPalette global)
        {
            let config_clone = Rc::clone(&config_rc);
            let window_weak = window.as_weak();
            window.on_toggle_theme(move || {
                let mut cfg = config_clone.borrow_mut();
                cfg.dark_mode = !cfg.dark_mode;
                let is_dark = cfg.dark_mode;
                if let Err(e) = cfg.save() {
                    eprintln!("[DeFiveK Shell] Warning saving config: {}", e);
                }

                if let Some(win) = window_weak.upgrade() {
                    // Update global palette first
                    let palette = win.global::<ShellPalette>();
                    palette.set_dark_mode(is_dark);

                    win.set_dark_mode(is_dark);
                    let theme_str = if is_dark { "Dark" } else { "Light" };
                    win.set_status_banner(SharedString::from(format!(
                        "Theme changed: {} Mode",
                        theme_str
                    )));
                }
            });
        }

        // 3. Shell Controls Visibility Toggle
        {
            let config_clone = Rc::clone(&config_rc);
            let window_weak = window.as_weak();
            window.on_toggle_shell_controls(move || {
                let mut cfg = config_clone.borrow_mut();
                cfg.show_shell_controls = !cfg.show_shell_controls;
                let visible = cfg.show_shell_controls;
                let _ = cfg.save();

                if let Some(win) = window_weak.upgrade() {
                    win.set_show_shell_controls(visible);
                }
            });
        }

        // 4. Desktop Quick Action Callback
        {
            let window_weak = window.as_weak();
            window.on_app_action(move |action_name| {
                eprintln!("[DeFiveK Shell] Action: {}", action_name);
                if let Some(win) = window_weak.upgrade() {
                    win.set_status_banner(SharedString::from(format!(
                        "Action: {}",
                        action_name
                    )));
                }
            });
        }

        // 5. File Explorer: Navigate to path
        {
            let window_weak = window.as_weak();
            let history_clone = Rc::clone(&history_stack);
            window.on_files_navigate(move |path_str| {
                let target = Self::resolve_path(&path_str);
                eprintln!("[DeFiveK Files] Navigating to: {:?}", target);

                if let Some(win) = window_weak.upgrade() {
                    let current = PathBuf::from(win.get_files_current_path().as_str());
                    history_clone.borrow_mut().push(current);

                    match Self::read_directory(&target) {
                        Ok(items) => {
                            let count = items.len();
                            let model = Rc::new(VecModel::from(items));
                            win.set_files_items(ModelRc::from(model));
                            win.set_files_current_path(SharedString::from(target.to_string_lossy().to_string()));
                            win.set_files_has_error(false);
                            win.set_files_status_text(SharedString::from(format!("{} items in folder", count)));
                        }
                        Err(err) => {
                            win.set_files_has_error(true);
                            win.set_files_error_text(SharedString::from(err));
                        }
                    }
                }
            });
        }

        // 6. File Explorer: Navigate Up (parent directory)
        {
            let window_weak = window.as_weak();
            let history_clone = Rc::clone(&history_stack);
            window.on_files_navigate_up(move || {
                if let Some(win) = window_weak.upgrade() {
                    let current = PathBuf::from(win.get_files_current_path().as_str());
                    if let Some(parent) = current.parent().map(|p| p.to_path_buf()) {
                        history_clone.borrow_mut().push(current);
                        match Self::read_directory(&parent) {
                            Ok(items) => {
                                let count = items.len();
                                let model = Rc::new(VecModel::from(items));
                                win.set_files_items(ModelRc::from(model));
                                win.set_files_current_path(SharedString::from(parent.to_string_lossy().to_string()));
                                win.set_files_has_error(false);
                                win.set_files_status_text(SharedString::from(format!("{} items in folder", count)));
                            }
                            Err(err) => {
                                win.set_files_has_error(true);
                                win.set_files_error_text(SharedString::from(err));
                            }
                        }
                    }
                }
            });
        }

        // 7. File Explorer: Navigate Back
        {
            let window_weak = window.as_weak();
            let history_clone = Rc::clone(&history_stack);
            window.on_files_navigate_back(move || {
                if let Some(win) = window_weak.upgrade() {
                    let prev = history_clone.borrow_mut().pop();
                    if let Some(prev_path) = prev {
                        match Self::read_directory(&prev_path) {
                            Ok(items) => {
                                let count = items.len();
                                let model = Rc::new(VecModel::from(items));
                                win.set_files_items(ModelRc::from(model));
                                win.set_files_current_path(SharedString::from(prev_path.to_string_lossy().to_string()));
                                win.set_files_has_error(false);
                                win.set_files_status_text(SharedString::from(format!("{} items in folder", count)));
                            }
                            Err(err) => {
                                win.set_files_has_error(true);
                                win.set_files_error_text(SharedString::from(err));
                            }
                        }
                    }
                }
            });
        }

        // 8. File Explorer: Refresh
        {
            let window_weak = window.as_weak();
            window.on_files_refresh(move || {
                if let Some(win) = window_weak.upgrade() {
                    let current = PathBuf::from(win.get_files_current_path().as_str());
                    match Self::read_directory(&current) {
                        Ok(items) => {
                            let count = items.len();
                            let model = Rc::new(VecModel::from(items));
                            win.set_files_items(ModelRc::from(model));
                            win.set_files_has_error(false);
                            win.set_files_status_text(SharedString::from(format!("Refreshed ({} items)", count)));
                        }
                        Err(err) => {
                            win.set_files_has_error(true);
                            win.set_files_error_text(SharedString::from(err));
                        }
                    }
                }
            });
        }

        // 9. File Explorer: Open Item (directory -> navigate, file -> open in Editor)
        {
            let window_weak = window.as_weak();
            let history_clone = Rc::clone(&history_stack);
            window.on_files_open_item(move |path_str, is_dir| {
                let target = PathBuf::from(path_str.as_str());
                if let Some(win) = window_weak.upgrade() {
                    if is_dir {
                        let current = PathBuf::from(win.get_files_current_path().as_str());
                        history_clone.borrow_mut().push(current);
                        match Self::read_directory(&target) {
                            Ok(items) => {
                                let count = items.len();
                                let model = Rc::new(VecModel::from(items));
                                win.set_files_items(ModelRc::from(model));
                                win.set_files_current_path(SharedString::from(target.to_string_lossy().to_string()));
                                win.set_files_has_error(false);
                                win.set_files_status_text(SharedString::from(format!("{} items in folder", count)));
                            }
                            Err(err) => {
                                win.set_files_has_error(true);
                                win.set_files_error_text(SharedString::from(err));
                            }
                        }
                    } else {
                        // Open file in Editor
                        match fs::read_to_string(&target) {
                            Ok(content) => {
                                win.set_edit_file_path(SharedString::from(target.to_string_lossy().to_string()));
                                win.set_edit_file_content(SharedString::from(content));
                                win.set_edit_status(SharedString::from(format!("Opened {:?}", target.file_name().unwrap_or_default())));
                                win.set_edit_is_modified(false);
                                win.set_status_banner(SharedString::from(format!("Opened file in Editor: {:?}", target.file_name().unwrap_or_default())));
                            }
                            Err(e) => {
                                win.set_status_banner(SharedString::from(format!("Cannot open file: {}", e)));
                            }
                        }
                    }
                }
            });
        }

        // 10. Terminal: Run Command
        {
            let window_weak = window.as_weak();
            window.on_term_run_command(move |cmd_str| {
                let trimmed = cmd_str.trim().to_string();
                if trimmed.is_empty() {
                    return;
                }

                if let Some(win) = window_weak.upgrade() {
                    let mut output_history = win.get_term_output().to_string();
                    output_history.push_str(&format!("harry@defivek:~$ {}\n", trimmed));

                    if trimmed == "help" {
                        output_history.push_str(
                            "DeFiveK Desktop Interactive Shell — Available Commands:\n\
                             • help                 Show this help manual\n\
                             • clear                Clear terminal buffer\n\
                             • ls [-la]             List files in directory\n\
                             • uname [-a]           Display kernel & architecture\n\
                             • whoami               Print current user\n\
                             • date                 Display current timestamp\n\
                             • pwd                  Show current directory\n\
                             • git status           Git repository status\n\
                             • cat <file>           Display text file content\n\
                             Or click 'Launch Host Terminal' to spawn external terminal emulator.\n\n"
                        );
                    } else if trimmed == "clear" {
                        output_history.clear();
                    } else {
                        // Run safely through sh -c
                        let execution = Command::new("sh")
                            .arg("-c")
                            .arg(&trimmed)
                            .output();

                        match execution {
                            Ok(out) => {
                                let stdout = String::from_utf8_lossy(&out.stdout);
                                let stderr = String::from_utf8_lossy(&out.stderr);
                                if !stdout.is_empty() {
                                    output_history.push_str(&stdout);
                                }
                                if !stderr.is_empty() {
                                    output_history.push_str(&stderr);
                                }
                                if stdout.is_empty() && stderr.is_empty() {
                                    output_history.push_str("(Command finished with no output)\n");
                                }
                            }
                            Err(err) => {
                                output_history.push_str(&format!("Error executing command: {}\n", err));
                            }
                        }
                        output_history.push('\n');
                    }

                    win.set_term_output(SharedString::from(output_history));
                }
            });
        }

        // 11. Terminal: Launch Host Terminal
        {
            let window_weak = window.as_weak();
            window.on_term_launch_host(move || {
                let (_, cmd) = Self::discover_host_terminal();
                eprintln!("[DeFiveK Terminal] Spawning host terminal: {}", cmd);

                let res = if cfg!(target_os = "macos") {
                    Command::new("open").arg("-a").arg("Terminal").spawn()
                } else {
                    Command::new(cmd).spawn()
                };

                if let Some(win) = window_weak.upgrade() {
                    match res {
                        Ok(_) => {
                            win.set_status_banner(SharedString::from("Launched external host terminal emulator"));
                        }
                        Err(e) => {
                            win.set_status_banner(SharedString::from(format!("Host terminal launch notice: {}", e)));
                        }
                    }
                }
            });
        }

        // 12. Terminal: Clear Output
        {
            let window_weak = window.as_weak();
            window.on_term_clear(move || {
                if let Some(win) = window_weak.upgrade() {
                    win.set_term_output(SharedString::from("DeFiveK Terminal buffer cleared.\n\n"));
                }
            });
        }

        // 13. Text Editor: New File
        {
            let window_weak = window.as_weak();
            window.on_edit_new(move || {
                if let Some(win) = window_weak.upgrade() {
                    win.set_edit_file_path(SharedString::from("Untitled.txt"));
                    win.set_edit_file_content(SharedString::from(""));
                    win.set_edit_status(SharedString::from("New document created"));
                    win.set_edit_is_modified(false);
                }
            });
        }

        // 14. Text Editor: Open Preset
        {
            let window_weak = window.as_weak();
            window.on_edit_open_preset(move |preset_name| {
                let base = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                let target = base.join(preset_name.as_str());

                if let Some(win) = window_weak.upgrade() {
                    match fs::read_to_string(&target) {
                        Ok(content) => {
                            win.set_edit_file_path(SharedString::from(target.to_string_lossy().to_string()));
                            win.set_edit_file_content(SharedString::from(content));
                            win.set_edit_status(SharedString::from(format!("Loaded {}", preset_name)));
                            win.set_edit_is_modified(false);
                        }
                        Err(e) => {
                            win.set_edit_status(SharedString::from(format!("Error loading {}: {}", preset_name, e)));
                        }
                    }
                }
            });
        }

        // 15. Text Editor: Save File
        {
            let window_weak = window.as_weak();
            window.on_edit_save(move |path_str, content_str| {
                let path = Self::resolve_path(&path_str);
                eprintln!("[DeFiveK Editor] Saving file: {:?}", path);

                if let Some(win) = window_weak.upgrade() {
                    match fs::write(&path, content_str.as_bytes()) {
                        Ok(_) => {
                            win.set_edit_status(SharedString::from(format!(
                                "✓ Successfully saved {} bytes to {:?}",
                                content_str.len(),
                                path.file_name().unwrap_or_default()
                            )));
                            win.set_edit_is_modified(false);
                            win.set_status_banner(SharedString::from("File saved successfully"));
                        }
                        Err(e) => {
                            win.set_edit_status(SharedString::from(format!("❌ Save failed: {}", e)));
                            win.set_status_banner(SharedString::from(format!("Save error: {}", e)));
                        }
                    }
                }
            });
        }

        // 16. Settings: Reset Defaults
        {
            let config_clone = Rc::clone(&config_rc);
            let window_weak = window.as_weak();
            window.on_settings_reset(move || {
                let mut cfg = config_clone.borrow_mut();
                *cfg = DeFiveKConfig::default();
                let _ = cfg.save();

                if let Some(win) = window_weak.upgrade() {
                    win.set_current_mode(cfg.mode.as_i32());
                    win.set_dark_mode(cfg.dark_mode);
                    win.set_show_shell_controls(cfg.show_shell_controls);
                    win.set_selected_wallpaper(SharedString::from(cfg.wallpaper.clone()));

                    let palette = win.global::<ShellPalette>();
                    palette.set_dark_mode(cfg.dark_mode);

                    win.set_status_banner(SharedString::from("Reset settings to default configuration"));
                }
            });
        }

        // 17. Wallpaper Personalization Selection Callback
        {
            let config_clone = Rc::clone(&config_rc);
            let window_weak = window.as_weak();
            window.on_wallpaper_selected(move |wp_id| {
                let wp_str = wp_id.to_string();
                eprintln!("[DeFiveK Shell] Wallpaper applied: {}", wp_str);

                let mut cfg = config_clone.borrow_mut();
                cfg.wallpaper = wp_str.clone();
                if let Err(e) = cfg.save() {
                    eprintln!("[DeFiveK Shell] Warning saving wallpaper preference: {}", e);
                }

                if let Some(win) = window_weak.upgrade() {
                    win.set_selected_wallpaper(SharedString::from(wp_str.clone()));
                    win.set_status_banner(SharedString::from(format!(
                        "Wallpaper updated: {}",
                        wp_str
                    )));
                }
            });
        }

        // 18. Periodic Clock & System Status Timer (1 second interval)
        let clock_timer = Timer::default();
        {
            let window_weak = window.as_weak();
            clock_timer.start(
                TimerMode::Repeated,
                Duration::from_secs(1),
                move || {
                    if let Some(win) = window_weak.upgrade() {
                        let now = Local::now();
                        win.set_clock_time(SharedString::from(now.format("%I:%M %p").to_string()));
                        win.set_clock_date(SharedString::from(now.format("%A, %b %e").to_string()));
                    }
                },
            );
        }

        Ok(Self {
            config: config_rc,
            window,
            _clock_timer: clock_timer,
            history_stack,
        })
    }

    /// Runs the Slint desktop shell event loop.
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!("[DeFiveK Shell] Starting desktop shell main window...");
        self.window.run()?;
        eprintln!("[DeFiveK Shell] Shell exited normally.");
        Ok(())
    }
}
