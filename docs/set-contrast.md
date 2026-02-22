# set-contrast / set-vcp - Set Monitor Color Settings

## SYNOPSIS

Adjust color and contrast settings on the monitor.

## SYNTAX

### Set contrast

```bash
monitorconfig set-contrast <VALUE> [OPTIONS]
```

### Set individual color values via VCP

```bash
monitorconfig set-vcp <CODE> <VALUE> [OPTIONS]
```

## DESCRIPTION

The Rust CLI provides color setting adjustment through multiple commands:

- `set-contrast` - Set contrast level (VCP code 0x12)
- `set-vcp` - Set individual color values using specific VCP codes

## COMMON COLOR VCP CODES

| Code | Name              | Description              | Typical Range     |
| ---- | ----------------- | ------------------------ | ----------------- |
| 0x12 | Contrast          | Contrast level           | 0-100             |
| 0x14 | Color Temperature | Color temperature        | Varies by monitor |
| 0x16 | Red Video Gain    | Red gain/tint            | 0-100             |
| 0x18 | Green Video Gain  | Green gain/tint          | 0-100             |
| 0x1A | Blue Video Gain   | Blue gain/tint           | 0-100             |
| 0x6C | Red Black Level   | Red drive/black level    | 0-100             |
| 0x6E | Green Black Level | Green drive/black level  | 0-100             |
| 0x70 | Blue Black Level  | Blue drive/black level   | 0-100             |
| 0x8A | Color Saturation  | Overall color saturation | 0-100             |

## ARGUMENTS

### `<VALUE>` (for set-contrast)

The contrast value to set (typically 0-100).

**Type:** Unsigned integer (u32)
**Required:** Yes

### `<CODE> <VALUE>` (for set-vcp)

The VCP code and value to set.

**Type:** u8, u32
**Required:** Yes

## OPTIONS

### `-d, --device <DEVICE>`

Specify the monitor by device name (e.g., `\\.\DISPLAY1`) or friendly name (e.g., "Dell U2723DE").

**Type:** String
**Required:** No (defaults to primary monitor if neither --device nor --primary is specified)

### `-p, --primary`

Target the primary monitor.

**Type:** Flag
**Required:** No

## EXAMPLES

### Example 1: Set contrast of primary monitor

```bash
monitorconfig set-contrast 60 --primary
```

Output:

```text
Contrast set to 60
```

### Example 2: Set contrast of specific monitor

```bash
monitorconfig set-contrast 75 --device "\\.\DISPLAY2"
```

### Example 3: Set contrast silently (for scripts)

```bash
monitorconfig set-contrast 50 --primary --silent
```

### Example 4: Set color values using VCP codes

```bash
# Set red video gain
monitorconfig set-vcp 0x16 50 --primary

# Set green video gain
monitorconfig set-vcp 0x18 50 --primary

# Set blue video gain
monitorconfig set-vcp 0x1A 50 --primary

# Set color temperature preset
monitorconfig set-vcp 0x14 11 --primary
```

### Example 5: Adjust contrast and save

```bash
# Set contrast
monitorconfig set-contrast 65 --primary

# Save settings to monitor's non-volatile memory
monitorconfig save-settings --primary
```

## NOTES

- Contrast values are typically in the range 0-100, but the actual range depends on the monitor hardware.
- The monitor must support DDC/CI and VCP code 0x12 (Contrast).
- If neither `--device` nor `--primary` is specified, the primary monitor is used by default.
- You can use either the device name (`\\.\DISPLAY1`) or the friendly name ("Dell U2723DE") with the `--device` option.
- Device name matching is case-insensitive.
- For color-related VCP codes, use `scan-vcp` to discover which codes your monitor supports and their valid ranges.

## SEE ALSO

- [get-contrast](get-contrast.md) - Get monitor contrast
- [set-brightness](set-brightness.md) - Set monitor brightness
- [set-vcp](set-vcp.md) - Set any VCP feature value
- [save-settings](save-settings.md) - Save current settings to monitor memory
- [list](list.md) - List all monitors
