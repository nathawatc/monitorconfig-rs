# Contributing to MonitorConfig

First off, thank you for considering contributing to MonitorConfig! It's people
like you that make open-source tools great.

This document provides guidelines and instructions for contributing to this
project.

## Project Overview

MonitorConfig is a native Windows CLI tool for controlling external monitors via
the DDC/CI protocol. It is written in Rust, focusing on high performance,
safety, and a minimal binary footprint.

## Prerequisites

Before you begin, ensure you have the following installed and set up:

1. **Rust Toolchain**: Rust 1.93.0 or higher (Edition 2024).
2. **Operating System**:
   - **Windows 10/11**: Recommended for native development and testing.
   - **Linux**: Supported for cross-compilation using MinGW-w64
     (`x86_64-pc-windows-gnu`).
3. **Hardware**: A physical external monitor with **DDC/CI enabled** in its OSD
   settings.
   - _Note: Laptop built-in displays do not support DDC/CI and commands will
     fail on them._

## Development Workflow

### 1. Clone the repository

```bash
git clone https://codeberg.org/nathawat/monitorconfig-rs.git
cd monitorconfig-rs
```

### 2. Build the project

For standard debug build:

```bash
cargo build
```

For release build (optimized):

```bash
cargo build --release
```

To build without the console window (for background tasks):

```bash
cargo build --release --features gui-subsystem
```

### 3. Code Style and Linting

We maintain strict code quality standards. Before submitting a pull request,
please ensure your code passes formatting and linting checks:

```bash
# Format the code
cargo fmt --all

# Run standard tests
cargo test

# Run clippy to check for common mistakes and warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

_Note: We treat all Clippy warnings as errors (`-D warnings`). Your PR will not
be accepted if it introduces new warnings._

## Testing Guidelines

Because this tool interacts directly with monitor hardware, automated
integration testing is difficult. **Manual hardware testing is required** for
any features related to DDC/CI communication.

Common commands for testing your changes locally:

```bash
# Verify monitor detection
cargo run -- list --json

# Test basic VCP read
cargo run -- get-brightness --primary

# Discover supported VCP codes on your monitor
cargo run -- scan-vcp --primary
```

## Architecture & Code Guidelines

When contributing new code, please adhere to the established design patterns:

- **Module Responsibilities**:
  - `cli.rs`: CLI routing, argument parsing (using `clap`), and command orchestration.
  - `monitor.rs`: High-level monitor representation (`PhysicalMonitor`), handle
    lifecycle, and enumeration logic.
  - `native.rs`: **All `unsafe` FFI bindings** to Windows APIs (`dxva2.dll`,
    `user32.dll`) must be isolated here.
  - `vcp.rs`: VESA Command Protocol definitions and specific feature
    implementations.
  - `error.rs`: Centralized error handling using `thiserror`.

- **Memory Management & FFI**:
  - All Windows API handles (`HMONITOR`, `HANDLE`) must be managed safely.
  - `PhysicalMonitor` implements the `Drop` trait to automatically call
    `DestroyPhysicalMonitor`. Ensure handles are not leaked during early returns
    or retry loops.
  - Document the `# Safety` preconditions for any new `unsafe fn` you create.

- **Error Handling**:
  - Do not use `.unwrap()` or `.expect()` in production code paths unless
    absolutely necessary (e.g., parsing constants).
  - Return `Result<T>` (aliased to `Result<T, MonitorError>`) and propagate
    errors gracefully.

- **VCP Values**:
  - Always format VCP codes as Hexadecimal internally and in CLI outputs
    (e.g., `0x10` for brightness).
  - All read-based CLI commands should support a `--json` flag.

## Submitting a Pull Request

1. **Fork** the repository and create your feature branch:
   `git checkout -b feature/my-new-feature`
2. **Commit** your changes with clear, descriptive commit messages.
3. **Push** to the branch: `git push origin feature/my-new-feature`
4. **Open a Pull Request** against the `main` branch.

In your PR description, please include:

- A summary of the changes and the problem they solve.
- Confirmation that you have run `cargo fmt` and `cargo clippy`.
- Details of the hardware you used to test the changes (Monitor model, Windows
  version).

We look forward to reviewing your contributions!
