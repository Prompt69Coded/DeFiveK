# DeFiveK Desktop — Configuration & Session Reference

## User Configuration File

DeFiveK Desktop stores runtime user preferences in a standard, platform-appropriate configuration directory:

* **Debian / Linux (XDG Base Directory)**:  
  `$XDG_CONFIG_HOME/defivek/config.toml`  
  *(defaults to `~/.config/defivek/config.toml` if `$XDG_CONFIG_HOME` is not set)*

* **macOS (Development & Testing)**:  
  `~/Library/Application Support/defivek/config.toml`

* **Environment Variable Override**:  
  You can override the path by setting `DEFIVEK_CONFIG_PATH`, for example:
  ```bash
  export DEFIVEK_CONFIG_PATH=/tmp/defivek-test-config.toml
  ```

### Configuration Format (TOML)

```toml
# DeFiveK Desktop Shell Configuration (Phase 1)
# Desktop mode: "windows", "macos", "android", or "aurora"
mode = "windows"

# Visual Theme: true for Dark mode, false for Light mode
dark_mode = true

# Show the top shared mode-switch bar
show_shell_controls = true

# Default logical dimensions
window_width = 1200
window_height = 760
```

### Fault Tolerance & Defaults

* If no configuration file exists at startup, DeFiveK Desktop automatically starts with default settings:
  * Mode: `windows` (Windows 7 inspired)
  * Dark mode: `true`
  * Shell switcher bar: `true`
* When settings are altered in the UI (switching modes or toggling themes), the configuration is automatically persisted to disk.
* If writing or reading fails (e.g., read-only filesystem), DeFiveK Desktop prints a warning message and continues running safely without crashing or panicking.

---

## LightDM Session File Reference (Documentation Only)

> [!WARNING]
> **DO NOT INSTALL THIS SESSION FILE IN PHASE 1.**  
> In Phase 1, DeFiveK Desktop runs as a normal standalone application within your existing X11 desktop environment (e.g., XFCE, GNOME, or Openbox under LightDM).  
> Dedicated LightDM session integration is planned for Phase 6.

An example session file is provided below for reference purposes only:

`config/defivek-session.desktop`:
```desktop
[Desktop Entry]
Name=DeFiveK Desktop (Phase 1 Preview)
Comment=Modular Linux Desktop Environment
Exec=/usr/local/bin/defivek
TryExec=/usr/local/bin/defivek
Icon=defivek-logo
Type=Application
DesktopNames=DeFiveK
```

In future phases (Phase 6), when system-level desktop session integration is implemented, this file can optionally be placed in `/usr/share/xsessions/defivek-session.desktop` by an explicit administrative installation step.
