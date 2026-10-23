# DeFiveK Desktop — Phase 3: Visual Excellence & Wallpaper Personalization

DeFiveK Desktop is a modular, visually exceptional Linux desktop environment designed for Debian GNU/Linux 13 (Trixie) ARM64 and macOS development. It features a unified Rust + Slint architecture supporting **four selectable desktop interaction paradigms**:

1. **Windows Mode**: Modernized Windows 7 Aero-inspired workflow with a frosted glass taskbar, Start launcher with real-time app search, system tray with calendar and status monitors, vector desktop shortcuts, and floating window management.
2. **macOS Mode**: Refined OS X Yosemite-inspired layout with a frosted top menu bar, interactive Apple menu, contextual application title, Control Center popover, and a floating dock with vector icons and active indicator dots.
3. **Android Mode**: Desktop-friendly Android 10-inspired interface with an At-a-Glance widget, pill search bar filtering an application drawer grid, quick settings dropdown with functional toggles, and 3-button navigation.
4. **Aurora Mode**: Original lightweight developer shell featuring a compact status panel, workspace tags (`1: DEV`, `2: WEB`, `3: SYS`, `4: MEDIA`), live resource monitors, a master-stack tiling split layout, and a Tiling vs Floating mode switch.

All four modes share the same core infrastructure, centralized design tokens, shared wallpaper engine, configuration persistence, and reactive light/dark theme system.

---

## What's New in Phase 3

- **Centralized Design System & Token Architecture (`theme.slint`)**:
  - Unified design tokens for neutral surfaces, cards, overlays, borders, typography hierarchy, brand and semantic accents (`success`, `warning`, `danger`, `info`).
  - Mode-specific signatures: Windows 7 Aero glass, macOS Yosemite frosted glass, Android Material surfaces, and Aurora cyber-cyan/purple accents.
  - Calibrated Light and Dark palettes designed deliberately for readability and contrast.
- **Shared Wallpaper Engine (`WallpaperView`)**:
  - Renders behind desktop content at Layer 0 across all four desktop modes.
  - Adapts to any display resolution and aspect ratio with non-distorting `image-fit: cover`.
  - Contrast scrim vignette ensuring desktop icons and text labels remain 100% legible over any wallpaper.
- **Curated Bundled Wallpapers**:
  - Eight original, lightweight, high-resolution vector SVG wallpapers bundled directly with the application:
    1. `aurora-borealis`: Deep midnight landscape with glowing waves of emerald, cyan, and violet aurora borealis over mountain silhouettes.
    2. `cosmic-nebula`: Deep interstellar dark violet space with delicate starlight geometry and magenta dust clouds.
    3. `alpine-sunset`: Multi-layered scenic alpine mountain ridges cascading against a sunset sky from twilight sapphire to amber-coral.
    4. `oceanic-flow`: Modern organic fluid wave ribbons in marine navy, oceanic teal, and turquoise.
    5. `minimal-geometry`: Architectural low-poly faceted crystalline structure in slate and ice-blue planes.
    6. `desert-dunes`: Minimalist rolling desert sand dunes with warm terracotta, bronze gold, and soft peach sunset lighting.
    7. `emerald-sanctuary`: Tranquil layered forest tree canopies and mountain mist with deep forest pine and emerald sage.
    8. `solar-radiance`: Vibrant modern morning solar gradient with amber, golden honey, and deep twilight magenta.
- **Dedicated Wallpaper Personalization in Settings**:
  - Live wallpaper preview frame with desktop mockup.
  - Interactive 8-item visual gallery with thumbnail previews, titles, descriptions, and active checkmark badges.
  - Instant background update when selected without disturbing open windows or state.
  - Automatic persistence to `~/.config/defivek/config.toml` retained across restarts.
  - Safe fallback to default if an asset is ever missing.
- **Cohesive Vector Icon System**:
  - Eliminates Unicode and raw emoji substitutes for essential system indicators.
  - Crisp, sharp vector icons for Computer, Folders, Files, Terminal, Editor, Settings, Info, Trash, Search, Wi-Fi, Volume, Battery, Bell, and OS Mode emblems.
- **Refined Window Chrome (`WindowFrame`)**:
  - Strict 36px titlebar height with subtle separator.
  - Clear visual distinction between active and inactive states.
  - Authentic macOS traffic lights with hover glyphs.
  - Clean Windows/Android/Aurora vector control buttons (minimize, maximize/restore, close with red hover state).
  - Corner resize handle with clean diagonal grip lines.
  - Draggable titlebar clamped safely within display boundaries.

---

## Target Environment & Execution Model

* **Target OS**: Debian GNU/Linux 13 (Trixie), ARM64 / aarch64
* **Host Hardware**: Apple Silicon Mac (UTM virtualization with VirGL 3D acceleration)
* **Display Server**: X11 session managed by LightDM (runs as an ordinary client application inside your existing session)
* **UI Toolkit**: [Slint](https://slint.dev/) 1.8 (Cargo resolves 1.18.1)
* **Configuration**: Serde + TOML stored in platform-standard XDG directories

---

## Build and Run Instructions

### Debian 13 (Trixie) ARM64 Prerequisites

```bash
# 1. Update package lists
sudo apt update

# 2. Install build essentials and Slint/X11 development headers
sudo apt install -y \
    build-essential \
    pkg-config \
    libx11-dev \
    libxcb1-dev \
    libxkbcommon-dev \
    libfontconfig-dev \
    libgl1-mesa-dev
```

### Build the Application

```bash
# Debug profile:
./scripts/build.sh

# Optimized release profile:
./scripts/build.sh --release
```

Or directly via Cargo:

```bash
cargo build --release
```

### Launch DeFiveK Desktop

```bash
./scripts/run.sh
```

Or directly via Cargo:

```bash
cargo run --release
```

---

## Installation & Using as a Normal Desktop Environment

DeFiveK Desktop can be used in two ways:
1. **As a full desktop application** inside your existing session (Windowed or Fullscreen).
2. **As a standalone X11 desktop session** selectable from LightDM at login.

### Method 1: Local User Installation (No root required)

Install the compiled release binary and desktop entry to `~/.local/bin`:

```bash
./scripts/install.sh
```

Ensure `~/.local/bin` is in your `PATH`. Then launch anytime:

```bash
# Windowed mode:
defivek

# Fullscreen Desktop Environment mode:
defivek --fullscreen
```

### Method 2: System-Wide LightDM Session (Selectable at Login)

To have DeFiveK Desktop appear in the session dropdown list on your LightDM login screen:

```bash
sudo ./scripts/install.sh --system
```

This installs:
- `/usr/local/bin/defivek` (optimized binary)
- `/usr/local/bin/defivek-session` (session launcher)
- `/usr/share/xsessions/defivek-session.desktop` (LightDM XSession entry)

**How to log into DeFiveK Desktop:**
1. Log out of your current desktop session.
2. At the LightDM greeter screen, click the session switcher icon (top right or near the username/password box).
3. Select **"DeFiveK Desktop Session"**.
4. Enter your password and log in.
5. To return to your standard desktop (XFCE, etc.), simply log out or restart and select your original session from the same LightDM menu.

---

## Configuration & Persistence

DeFiveK Desktop saves your active desktop mode, theme selection, and shell preferences automatically:

* **Linux location**: `~/.config/defivek/config.toml` (or `$XDG_CONFIG_HOME/defivek/config.toml`)
* **macOS location**: `~/Library/Application Support/defivek/config.toml`

### Example `config.toml`:

```toml
mode = "windows"
dark_mode = true
show_shell_controls = true
window_width = 1200
window_height = 760
```
