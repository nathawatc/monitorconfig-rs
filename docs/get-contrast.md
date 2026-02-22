# get-contrast / get-vcp - Get Monitor Color Settings

## SYNOPSIS

Retrieve color and contrast information from the monitor.

## SYNTAX

### Get contrast

```bash
monitorconfig get-contrast [OPTIONS]
```

### Get individual color values via VCP

```bash
monitorconfig get-vcp <CODE> [OPTIONS]
```

## DESCRIPTION

The Rust CLI provides color setting access through multiple commands:

- `get-contrast` - Get contrast level (VCP code 0x12)
- `get-vcp` - Get individual color values using specific VCP codes

## COMMON COLOR VCP CODES

| Code | Name              | Description                 |
| ---- | ----------------- | --------------------------- |
| 0x12 | Contrast          | Contrast level              |
| 0x14 | Color Temperature | Color temperature selection |
| 0x16 | Red Video Gain    | Red gain/tint               |
| 0x18 | Green Video Gain  | Green gain/tint             |
| 0x1A | Blue Video Gain   | Blue gain/tint              |
| 0x6C | Red Black Level   | Red drive/black level       |
| 0x6E | Green Black Level | Green drive/black level     |
| 0x70 | Blue Black Level  | Blue drive/black level      |

## OPTIONS

### `-d, --device <DEVICE>`

Specify the monitor by device name or friendly name.

**Type:** String
**Required:** No (defaults to primary monitor if neither --device nor --primary is specified)

### `-p, --primary`

Target the primary monitor.

**Type:** Flag
**Required:** No

### `-j, --json`

Output in JSON format.

**Type:** Flag
**Required:** No

## EXAMPLES

### Example 1: Get contrast of primary monitor

```bash
monitorconfig get-contrast --primary
```

Output:

```text
Current contrast: 50 (min: 0, max: 100)
```

### Example 2: Get contrast of specific monitor

```bash
monitorconfig get-contrast --device "\\.\DISPLAY2"
```

### Example 3: Get contrast in JSON format

```bash
monitorconfig get-contrast --primary --json
```

Output:

```json
{
  "minimum": 0,
  "current": 50,
  "maximum": 100
}
```

### Example 4: Get color values using VCP codes

```bash
# Red video gain
monitorconfig get-vcp 0x16 --primary

# Green video gain
monitorconfig get-vcp 0x18 --primary

# Blue video gain
monitorconfig get-vcp 0x1A --primary
```

## OUTPUT

### Text Format (Default)

Displays contrast information in human-readable format:

```text
Current contrast: <value> (min: <min>, max: <max>)
```

### JSON Format (--json)

Returns contrast information as a JSON object:

- `current` (number): Current contrast level
- `minimum` (number): Minimum supported contrast level
- `maximum` (number): Maximum supported contrast level

## NOTES

- Contrast values are typically in the range 0-100, but this depends on the monitor.
- The monitor must support DDC/CI and VCP code 0x12 (Contrast).
- If neither `--device` nor `--primary` is specified, the primary monitor is used by default.
- You can use either the device name (`\\.\DISPLAY1`) or the friendly name ("Dell U2723DE") with the `--device` option.
- Device name matching is case-insensitive.

## SEE ALSO

- [set-contrast](set-contrast.md) - Set monitor contrast
- [get-brightness](get-brightness.md) - Get monitor brightness
- [get-vcp](get-vcp.md) - Get any VCP feature value
- [list](list.md) - List all monitors
