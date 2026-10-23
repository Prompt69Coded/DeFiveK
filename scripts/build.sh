#!/usr/bin/env bash
# ==============================================================================
# DeFiveK Desktop — Build Script (Phase 1)
# Portable build script for Debian 13 ARM64 and macOS development
# ==============================================================================

set -euo pipefail

# Locate project root directory regardless of current working directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

echo "======================================================="
echo "  DeFiveK Desktop — Build System (Phase 1)"
echo "  Project Root: $PROJECT_ROOT"
echo "======================================================="

# 1. Check for Cargo / Rust toolchain
if ! command -v cargo >/dev/null 2>&1; then
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo ""
    echo "[ERROR] 'cargo' was not found in your PATH."
    echo ""
    echo "To install Rust and Cargo on Debian GNU/Linux 13 (Trixie):"
    echo "  1. Run the official rustup installer:"
    echo "     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "  2. Load the environment in your current terminal:"
    echo "     source \"\$HOME/.cargo/env\""
    echo "  3. Re-run this build script."
    echo ""
    exit 1
fi

RUST_VER="$(cargo --version)"
echo "[INFO] Using: $RUST_VER"

# 2. Platform inspection and dependency hints
OS_TYPE="$(uname -s)"
ARCH_TYPE="$(uname -m)"
echo "[INFO] Target Host: $OS_TYPE ($ARCH_TYPE)"

if [ "$OS_TYPE" = "Linux" ]; then
    echo "[INFO] Running on Linux. Verifying native dependencies..."
    if [ -f /etc/debian_version ]; then
        DEB_VER="$(cat /etc/debian_version)"
        echo "[INFO] Debian version: $DEB_VER"
        echo "[NOTE] Required Debian 13 ARM64 system packages:"
        echo "       build-essential libx11-dev libxcb1-dev libxkbcommon-dev libfontconfig-dev libgl1-mesa-dev"
        echo "       If build fails with missing library headers, install via:"
        echo "       sudo apt install -y build-essential libx11-dev libxcb1-dev libxkbcommon-dev libfontconfig-dev libgl1-mesa-dev"
    fi
fi

# 3. Determine build profile
BUILD_MODE="debug"
CARGO_FLAGS=()

for arg in "$@"; do
    case "$arg" in
        --release)
            BUILD_MODE="release"
            CARGO_FLAGS+=("--release")
            ;;
        --help|-h)
            echo "Usage: ./scripts/build.sh [--release]"
            exit 0
            ;;
    esac
done

echo ""
echo "[1/2] Running cargo check..."
cargo check "${CARGO_FLAGS[@]}"

echo ""
echo "[2/2] Compiling DeFiveK Desktop ($BUILD_MODE mode)..."
cargo build "${CARGO_FLAGS[@]}"

echo ""
echo "======================================================="
if [ "$BUILD_MODE" = "release" ]; then
    BINARY_PATH="$PROJECT_ROOT/target/release/defivek"
else
    BINARY_PATH="$PROJECT_ROOT/target/debug/defivek"
fi

if [ -f "$BINARY_PATH" ]; then
    echo "  Build SUCCESSFUL!"
    echo "  Binary generated at: $BINARY_PATH"
    echo "  You can launch the shell using:"
    echo "    ./scripts/run.sh"
else
    echo "  Build finished with target."
fi
echo "======================================================="
