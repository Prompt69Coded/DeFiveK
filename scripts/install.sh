#!/usr/bin/env bash
# ==============================================================================
# DeFiveK Desktop — Installation Script
# Supports:
#   1. Local User Install (no root required): installs to ~/.local/bin
#   2. System Session Install (with sudo): installs XSession to /usr/share/xsessions
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

echo "======================================================="
echo "  DeFiveK Desktop — Installation Utility"
echo "  Project Root: $PROJECT_ROOT"
echo "======================================================="

# Ensure Cargo is available
if ! command -v cargo >/dev/null 2>&1; then
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "[ERROR] cargo not found. Please ensure Rust is installed."
    exit 1
fi

# 1. Build optimized release binary
echo ""
echo "[1/3] Building optimized release binary..."
cargo build --release

RELEASE_BIN="$PROJECT_ROOT/target/release/defivek"
if [ ! -f "$RELEASE_BIN" ]; then
    echo "[ERROR] Binary not found at $RELEASE_BIN"
    exit 1
fi

# 2. Check installation target
INSTALL_SYSTEM=false
if [ "${1:-}" = "--system" ] || [ "$(id -u)" -eq 0 ]; then
    INSTALL_SYSTEM=true
fi

if [ "$INSTALL_SYSTEM" = true ]; then
    echo ""
    echo "[2/3] Installing System-Wide Desktop Session (requires root)..."
    
    install -d /usr/local/bin
    install -m 755 "$RELEASE_BIN" /usr/local/bin/defivek
    install -m 755 "$PROJECT_ROOT/scripts/defivek-session" /usr/local/bin/defivek-session

    install -d /usr/share/applications
    install -m 644 "$PROJECT_ROOT/defivek.desktop" /usr/share/applications/defivek.desktop

    install -d /usr/share/xsessions
    install -m 644 "$PROJECT_ROOT/defivek-session.desktop" /usr/share/xsessions/defivek-session.desktop

    echo ""
    echo "[3/3] Installation Complete!"
    echo "======================================================="
    echo "  System installation successful!"
    echo "  - Binary: /usr/local/bin/defivek"
    echo "  - Session: /usr/local/bin/defivek-session"
    echo "  - LightDM XSession: /usr/share/xsessions/defivek-session.desktop"
    echo ""
    echo "To use DeFiveK Desktop:"
    echo "  A) Log out of your current session."
    echo "  B) At the LightDM login screen, click the session icon/dropdown"
    echo "     and select 'DeFiveK Desktop Session'."
    echo "  C) Log in to start your DeFiveK Desktop environment!"
    echo "======================================================="
else
    echo ""
    echo "[2/3] Installing for current user (~/.local/bin)..."
    
    mkdir -p "$HOME/.local/bin"
    cp -f "$RELEASE_BIN" "$HOME/.local/bin/defivek"
    cp -f "$PROJECT_ROOT/scripts/defivek-session" "$HOME/.local/bin/defivek-session"
    chmod +x "$HOME/.local/bin/defivek" "$HOME/.local/bin/defivek-session"

    mkdir -p "$HOME/.local/share/applications"
    cp -f "$PROJECT_ROOT/defivek.desktop" "$HOME/.local/share/applications/defivek.desktop"

    echo ""
    echo "[3/3] Installation Complete!"
    echo "======================================================="
    echo "  User installation successful!"
    echo "  - Binary: $HOME/.local/bin/defivek"
    echo "  - Session launcher: $HOME/.local/bin/defivek-session"
    echo "  - Desktop shortcut: $HOME/.local/share/applications/defivek.desktop"
    echo ""
    echo "How to use:"
    echo "  1. Run inside your current desktop session:"
    echo "     defivek              # standard window mode"
    echo "     defivek --fullscreen # fullscreen desktop mode"
    echo ""
    echo "  2. To add DeFiveK Desktop to your LightDM login menu, run:"
    echo "     sudo ./scripts/install.sh --system"
    echo "======================================================="
fi
