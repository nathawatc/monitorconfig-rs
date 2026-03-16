# MonitorConfig - Rust CLI

A native Windows CLI for managing monitor settings via DDC/CI.
A complete Rust rewrite of the original PowerShell module.

## Features

- **List Monitors**: Enumerate all connected monitors.
- **Brightness & Contrast**: Get and set levels.
- **VCP Support**: Full VESA Command Protocol access.
- **Capabilities**: Query monitor capabilities string.
- **Settings**: Save or reset to factory defaults.
- **JSON Output**: For easy scripting.
- **Resume-safe**: `--wait` and `--retry` for post-sleep automation.

## Requirements

- Windows 10/11
- Rust 1.93.0+ (Edition 2024)
- DDC/CI enabled monitors

## Building

### Build Release

```bash
cargo build --release
```

Binary: `target/release/monitorconfig.exe`

### Install Globally

```bash
cargo install --path .
```

## Usage

### Examples

```bash
# List monitors
monitorconfig list --json

# Set primary brightness
monitorconfig set-brightness 50 --primary

# Set specific monitor contrast
monitorconfig set-contrast 60 --device "\\.\\DISPLAY1"

# Get/Set VCP (0x10 = Brightness)
monitorconfig get-vcp 0x10 --primary
monitorconfig set-vcp 0xD6 4 --primary # Power Off
```

#### `set-vcp` Options

| Option            | Short | Default | Description            |
| ----------------- | ----- | ------- | ---------------------- |
| `--device <NAME>` | `-d`  | —       | Target by path or name |
| `--primary`       | `-p`  | —       | Target primary monitor |
| `--wait <SEC>`    | `-w`  | `0`     | Delay before command   |
| `--retry <COUNT>` | —     | `0`     | Retries on failure     |
| `--retry-delay`   | —     | `5`     | Delay between retries  |
| `--silent`        | `-s`  | —       | Suppress output        |

> **Note on `--wait`**: Useful after resume from sleep. Some monitors need
> 10-15s to initialize DDC/CI after signal return. Using `--wait` prevents
> commands from being sent too early, which can hang the monitor MCU.

### VCP Codes

```bash
# Reference list
monitorconfig list-vcp

# Scan supported codes on monitor
monitorconfig scan-vcp --primary
```

## Background Tasks / Task Scheduler

Use `--silent`, `--wait`, and `--retry` for reliable automation.

### Resume-safe Pattern

```bash
# Delay 10s and retry 3 times if monitor is busy
monitorconfig set-vcp -d "MonitorName" 0x14 11 -s --wait 10 --retry 3
```

**Task Scheduler Tips:**

- **Trigger Delay**: Use small delay (2s); handle real delay with `--wait`.
- **Execution Limit**: Set to ~2m to allow for retries.
- **Instances**: Set to `IgnoreNew`.

### Hide Console Window

Build with GUI subsystem to prevent the console flash:

```bash
cargo build --release --features gui-subsystem
```

_Note: GUI builds produce no console output or errors._

## Common VCP Codes

| Code | Name       | Description               |
| ---- | ---------- | ------------------------- |
| 0x10 | Brightness | Image luminance           |
| 0x12 | Contrast   | Image contrast            |
| 0x14 | Color Temp | Select preset             |
| 0x60 | Input      | Select input source       |
| 0xD6 | Power      | Power state (1=On, 4=Off) |

## Technical Details

### Architecture

- **native**: Windows API bindings (dxva2, user32).
- **monitor**: Abstraction and enumeration.
- **vcp**: Protocol implementation.
- **cli**: Interface via `clap`.

### DDC/CI Support

Allows software control over I2C. Enable in OSD if needed. Some docks or
cables (HDMI) may have limited support compared to DisplayPort.

## Migration from PowerShell

| PowerShell Cmdlet        | Rust CLI Command               |
| ------------------------ | ------------------------------ |
| `Get-Monitor`            | `monitorconfig list`           |
| `Get-MonitorBrightness`  | `monitorconfig get-brightness` |
| `Set-MonitorBrightness`  | `monitorconfig set-brightness` |
| `Get-MonitorVCPResponse` | `monitorconfig get-vcp`        |
| `Set-MonitorVCPValue`    | `monitorconfig set-vcp`        |

## License

Same as the original MonitorConfig PowerShell module.

## Acknowledgments

Based on [MonitorConfig](https://github.com/MartinGC94/MonitorConfig)
by Martin Gräßlin.
