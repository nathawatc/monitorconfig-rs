# set-brightness - Set Monitor Brightness

## SYNOPSIS

Sets the brightness level on the specified monitor.

## SYNTAX

```bash
monitorconfig set-brightness <VALUE> [OPTIONS]
```

## DESCRIPTION

The `set-brightness` command adjusts the brightness level of a monitor to the specified value. This uses VCP code 0x10 (Brightness) internally.

## ARGUMENTS

### `<VALUE>`

The brightness value to set (typically 0-100).

**Type:** Unsigned integer (u32)
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

### Example 1: Set brightness of primary monitor

```bash
monitorconfig set-brightness 75 --primary
```

Output:

```text
Brightness set to 75
```

### Example 2: Set brightness of specific monitor

```bash
monitorconfig set-brightness 50 --device "\\.\DISPLAY2"
```

### Example 3: Set all monitors to the same brightness

```bash
# Set brightness for each monitor individually
monitorconfig set-brightness 80 --device "\\.\DISPLAY1"
monitorconfig set-brightness 80 --device "\\.\DISPLAY2"
```

### Example 4: Set brightness silently (for scripts)

```bash
monitorconfig set-brightness 50 --primary --silent
```

## NOTES

- Brightness values are typically in the range 0-100, but the actual range depends on the monitor hardware.
- The monitor must support DDC/CI and VCP code 0x10 (Brightness).
- If neither `--device` nor `--primary` is specified, the primary monitor is used by default.
- You can use either the device name (`\\.\DISPLAY1`) or the friendly name ("Dell U2723DE") with the `--device` option.
- Device name matching is case-insensitive.

## SEE ALSO

- [get-brightness](get-brightness.md) - Get monitor brightness
- [set-contrast](set-contrast.md) - Set monitor contrast
- [set-vcp](set-vcp.md) - Set any VCP feature value
- [list](list.md) - List all monitors
