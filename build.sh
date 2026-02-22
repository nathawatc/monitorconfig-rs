#!/bin/bash
# Build script for monitorconfig (Linux → Windows cross-compilation)

set -e

echo "Building MonitorConfig Rust CLI..."
echo "=================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

# Show Rust version
echo "Rust version:"
rustc --version
cargo --version
echo ""

# Check if cross-compilation toolchain is installed
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Warning: MinGW-w64 cross-compilation toolchain not found."
    echo "To install, run:"
    echo "  sudo apt-get install gcc-mingw-w64-x86-64 -y  # Debian/Ubuntu"
    echo "  or use your distro's package manager (e.g., pacman -S mingw-w64-x86_64-gcc for Arch)"
    echo ""
fi

# Add Windows target if not already installed
echo "Adding Windows target..."
rustup target add x86_64-pc-windows-gnu
echo ""

# Build in release mode
if [[ "$1" == "--gui" || "$1" == "gui" ]]; then
    echo "Building with GUI subsystem (no console window)..."
    cargo build --target x86_64-pc-windows-gnu --release --features gui-subsystem
    GUI_BUILD=true
else
    echo "Building standard version for Windows..."
    cargo build --target x86_64-pc-windows-gnu --release
    GUI_BUILD=false
fi

BINARY="target/x86_64-pc-windows-gnu/release/monitorconfig.exe"

echo ""
echo "Build complete!"
echo ""
echo "Binary location: ${BINARY}"
echo "Binary size:"
ls -lh "${BINARY}" | awk '{print $5}'
echo ""

if [ "$GUI_BUILD" = true ]; then
    echo "Note: This build uses GUI subsystem and won't show console windows."
    echo ""
fi

# Verify the binary was produced
if [ -f "${BINARY}" ]; then
    echo "Build verified: ${BINARY} exists."
    file "${BINARY}"
else
    echo "Error: Expected binary not found at ${BINARY}"
    exit 1
fi

echo ""
echo "To install globally, run: cargo install --path ."
echo "For GUI subsystem build, run: ./build.sh --gui"
echo ""
echo "Note: This is a Windows binary. Copy it to a Windows machine to run it,"
echo "or use Wine: wine ${BINARY} --version"
echo ""
echo "To test the tool on Windows, run: monitorconfig.exe list"
