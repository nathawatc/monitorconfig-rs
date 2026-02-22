# get-vcp / scan-vcp - Query VCP Features

## SYNOPSIS

Query monitor VCP (VESA Command Protocol) feature codes.

## SYNTAX

### Get specific VCP code

```bash
monitorconfig get-vcp <CODE> [OPTIONS]
```

### Scan all supported VCP codes

```bash
monitorconfig scan-vcp [OPTIONS]
```

## DESCRIPTION

These commands let you query monitors for VCP (VESA Command Protocol) feature values.

- `get-vcp` retrieves the current and maximum value for a specific VCP code
- `scan-vcp` scans the monitor for all supported VCP codes (tests codes 0x00-0xFF)

## ARGUMENTS (get-vcp)

### `<CODE>`

The VCP code to query (0-255, can be specified in hex like `0x10` or decimal like `16`).

**Type:** Unsigned byte (u8)
**Required:** Yes (for get-vcp only)

## OPTIONS

### `-d, --device <DEVICE>`

Specify the monitor by device name (e.g., `\\.\DISPLAY1`) or friendly name (e.g., "Dell U2723DE").

**Type:** String
**Required:** No (defaults to primary monitor)

### `-p, --primary`

Target the primary monitor.

**Type:** Flag
**Required:** No

### `-j, --json`

Output in JSON format.

**Type:** Flag
**Required:** No

## EXAMPLES

### Example 1: Get brightness using VCP code

```bash
monitorconfig get-vcp 0x10 --primary
```

Output:

```text
VCP Code: 0x10 - Luminance
Description: Luminance of the image (Brightness control).
Current value: 75 (max: 100)
Type: SetParameter
```

### Example 2: Get input source

```bash
monitorconfig get-vcp 0x60 --device "\\.\DISPLAY1"
```

### Example 3: Get VCP value in JSON format

```bash
monitorconfig get-vcp 0x10 --primary --json
```

Output:

```json
{
  "vcp_code": 16,
  "current_value": 75,
  "maximum_value": 100,
  "code_type": "SetParameter"
}
```

### Example 4: Scan all supported VCP codes

```bash
monitorconfig scan-vcp --primary
```

Output:

```text
Scanning monitor for supported VCP codes...
Found 15 supported VCP codes

Code   Name                                CurrentValue MaxValue Description
------------------------------------------------------------------------------------------------------------------------
0x10   Luminance                           75           100      Luminance of the image (Brightness control).
0x12   Contrast                            50           100      Contrast of the image.
0x14   Select Color Preset                 6            11       Select a specified color temperature.
0x60   Input Select                        3            4        Adjusts the active input on the display.
...
```

### Example 5: Scan VCP codes in JSON format

```bash
monitorconfig scan-vcp --primary --json
```

## OUTPUT

### Text Format (Default)

**get-vcp** displays the VCP code, its name and description (if known), current value, maximum value, and type (`SetParameter` or `Momentary`).

**scan-vcp** displays a table of all supported VCP codes with their current and maximum values.

### JSON Format (--json)

**get-vcp** returns a single object with fields:

- `vcp_code` (number): The VCP code queried
- `current_value` (number): Current value
- `maximum_value` (number): Maximum value
- `code_type` (string): `"SetParameter"` or `"Momentary"`

**scan-vcp** returns an array of the same objects.

## NOTES

- VCP code support varies by monitor model. Use `scan-vcp` to discover which codes your monitor supports.
- VCP codes can be specified in hex (`0x10`, `0X10`) or decimal (`16`).
- The `code_type` field indicates whether the code is a continuous setting (`SetParameter`) or a one-shot action (`Momentary`).
- DDC/CI must be enabled in the monitor's OSD (On-Screen Display) settings.

## SEE ALSO

- [set-vcp](set-vcp.md) - Set a VCP feature value
- [list-vcp](../README.md#list-known-vcp-codes) - List all known VCP codes
- [list](list.md) - List all monitors
