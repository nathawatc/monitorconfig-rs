#!/bin/bash
# Build script for monitorconfig

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

echo ""
echo "Build complete!"
echo ""
echo "Binary location: target/x86_64-pc-windows-gnu/release/monitorconfig.exe"
echo "Binary size:"
ls -lh target/x86_64-pc-windows-gnu/release/monitorconfig.exe | awk '{print $5}'
echo ""

if [ "$GUI_BUILD" = true ]; then
    echo "Note: This build uses GUI subsystem and won't show console windows."
    echo ""
fi

echo "To install globally, run: cargo install --path ."
echo "For GUI subsystem build, run: ./build.sh --gui"
echo ""

# Run basic check
echo "Running basic functionality check..."
echo ""
./target/x86_64-pc-windows-gnu/release/monitorconfig.exe --version
echo ""
echo "To test the tool, run: ./target/x86_64-pc-windows-gnu/release/monitorconfig.exe list"
