# list - List Available Monitors

## SYNOPSIS

Enumerates all active physical monitors connected to the system.

## SYNTAX

```bash
monitor-config list [OPTIONS]
```

## DESCRIPTION

The `list` command enumerates all physical monitors that are currently active members of the Windows desktop. It displays information about each monitor including device name, friendly name, and whether it's the primary monitor.

## OPTIONS

### `-j, --json`

Output monitor information in JSON format.

**Type:** Flag  
**Required:** No

## EXAMPLES

### Example 1: List all monitors in table format

```bash
monitor-config list
```

Displays all connected monitors in a human-readable table format:

```text
Device Name          Friendly Name                  Primary
----------------------------------------------------------------------
\\.\DISPLAY1         Dell U2723DE                   Yes
\\.\DISPLAY2         Samsung S27F350                
```

### Example 2: List monitors in JSON format

```bash
monitor-config list --json
```

Outputs monitor information as JSON for scripting:

```json
[
  {
    "device_name": "\\.\\DISPLAY1",
    "friendly_name": "Dell U2723DE",
    "is_primary": true
  },
  {
    "device_name": "\\.\\DISPLAY2",
    "friendly_name": "Samsung S27F350",
    "is_primary": false
  }
]
```

### Example 3: List monitors and set brightness on the first one

```bash
# First, list monitors to see their device names
monitor-config list

# Then set brightness on a specific monitor
monitor-config set-brightness 50 --device "\\.\DISPLAY1"
```

## OUTPUT

### Table Format (Default)

Displays monitors in a formatted table with columns:

- **Device Name**: Windows device identifier (e.g., `\\.\DISPLAY1`)
- **Friendly Name**: Human-readable monitor name from manufacturer
- **Primary**: Indicates if this is the primary display

### JSON Format (--json)

Returns an array of monitor objects with fields:

- `device_name` (string): Windows device identifier
- `friendly_name` (string): Monitor manufacturer name
- `is_primary` (boolean): Whether this is the primary display

## NOTES

- Device names follow the pattern `\\.\DISPLAYX` where X is a number starting from 1
- The numbering may differ from what Windows display settings show
- All monitors are accessed via DDC/CI protocol (no WMI support in Rust version)
- Monitors must support DDC/CI and have it enabled in their OSD settings

## SEE ALSO

- [get-brightness](Get-MonitorBrightness.md) - Get monitor brightness
- [set-brightness](Set-MonitorBrightness.md) - Set monitor brightness
- [get-capabilities](Get-MonitorDetails.md) - Get monitor capabilities

### MartinGC94.MonitorConfig.API.VCP.VCPMonitor

Returned if the monitor cannot be controlled through WMI. DDC/CI support is not guaranteed.

## NOTES

## RELATED LINKS
