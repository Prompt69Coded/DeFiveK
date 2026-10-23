#!/usr/bin/env bash
# ==============================================================================
# DeFiveK Desktop — Run Script (Phase 1)
# Launches the desktop shell prototype within the active desktop session.
# ==============================================================================

set -euo pipefail

# Locate project root directory regardless of current working directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# 1. Ensure Cargo / Rust toolchain is available
if ! command -v cargo >/dev/null 2>&1; then
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "[ERROR] 'cargo' command not found in PATH."
    echo "Please install Rust using: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "and run: source \"\$HOME/.cargo/env\""
    exit 1
fi

# 2. Linux & X11 Display Server check
OS_TYPE="$(uname -s)"
if [ "$OS_TYPE" = "Linux" ]; then
    if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then
        echo "[WARNING] Neither DISPLAY nor WAYLAND_DISPLAY is set."
        echo "DeFiveK Desktop runs as a graphical application inside your active desktop session (LightDM/X11)."
        echo "If running over SSH, ensure X11 forwarding (-X) is enabled, or launch from an in-guest terminal."
    else
        echo "[INFO] Display detected: DISPLAY='${DISPLAY:-none}', WAYLAND_DISPLAY='${WAYLAND_DISPLAY:-none}'"
    fi
fi

# 3. Check VirGL / Hardware Acceleration hint
if [ "$OS_TYPE" = "Linux" ]; then
    echo "[INFO] Running on Linux ($OS_TYPE). VirGL graphics acceleration is recommended in UTM."
fi

# 4. Launch DeFiveK Desktop using cargo run
RUN_FLAGS=()
for arg in "$@"; do
    case "$arg" in
        --release)
            RUN_FLAGS+=("--release")
            ;;
        --help|-h)
            echo "Usage: ./scripts/run.sh [--release]"
            exit 0
            ;;
    esac
done

echo "[INFO] Launching DeFiveK Desktop Shell Prototype..."
cargo run "${RUN_FLAGS[@]}"
