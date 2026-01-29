#!/bin/bash
# Build script for monitor-config

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

# Build in release mode
echo "Building in release mode..."
cargo build --release

echo ""
echo "Build complete!"
echo ""
echo "Binary location: target/release/monitor-config.exe"
echo "Binary size:"
ls -lh target/release/monitor-config.exe | awk '{print $5}'
echo ""
echo "To install globally, run: cargo install --path ."
echo ""

# Run basic check
echo "Running basic functionality check..."
echo ""
./target/release/monitor-config.exe --version
echo ""
echo "To test the tool, run: ./target/release/monitor-config.exe list"
