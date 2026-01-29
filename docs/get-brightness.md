# get-brightness - Get Monitor Brightness

## SYNOPSIS

Returns brightness details (minimum, maximum, and current value) for the specified monitor.

## SYNTAX

```bash
monitorconfig get-brightness [OPTIONS]
```

## DESCRIPTION

The `get-brightness` command retrieves the current brightness level along with the minimum and maximum supported values for a monitor. This uses VCP code 0x10 (Brightness) internally.

## OPTIONS

### `-d, --device <DEVICE>`

Specify the monitor by device name (e.g., `\\.\DISPLAY1`) or friendly name (e.g., "Dell U2723DE").

**Type:** String  
**Required:** No (defaults to primary monitor if neither --device nor --primary is specified)

### `-p, --primary`

Target the primary monitor.

**Type:** Flag  
**Required:** No

### `-j, --json`

Output brightness information in JSON format.

**Type:** Flag  
**Required:** No

## EXAMPLES

### Example 1: Get brightness of primary monitor

```bash
monitorconfig get-brightness --primary
```

Output:

```text
Current brightness: 75 (min: 0, max: 100)
```

### Example 2: Get brightness of specific monitor

```bash
monitorconfig get-brightness --device "\\.\DISPLAY2"
```

### Example 3: Get brightness in JSON format

```bash
monitorconfig get-brightness --primary --json
```

Output:

```json
{
  "current": 75,
  "minimum": 0,
  "maximum": 100
}
```

### Example 4: Get brightness using friendly name

```bash
monitorconfig get-brightness --device "Dell U2723DE"
```

## OUTPUT

### Text Format (Default)

Displays brightness information in human-readable format:

```text
Current brightness: <value> (min: <min>, max: <max>)
```

### JSON Format (--json)

Returns brightness information as a JSON object:

- `current` (number): Current brightness level
- `minimum` (number): Minimum supported brightness level
- `maximum` (number): Maximum supported brightness level

## NOTES

- Brightness values are typically in the range 0-100, but this depends on the monitor
- The monitor must support DDC/CI and VCP code 0x10 (Brightness)
- If neither `--device` nor `--primary` is specified, the primary monitor is used by default
- You can use either the device name (`\\.\DISPLAY1`) or the friendly name ("Dell U2723DE") with the `--device` option

## SEE ALSO

- [set-brightness](Set-MonitorBrightness.md) - Set monitor brightness
- [get-contrast](Set-MonitorBrightness.md#get-contrast) - Get monitor contrast
- [get-vcp](Get-MonitorVCPResponse.md) - Get any VCP feature value
- [list](Get-Monitor.md) - List all monitors
