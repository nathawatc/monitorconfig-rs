# reset-defaults - Reset Monitor to Factory Defaults

## SYNOPSIS

Resets monitor settings to factory default values.

## SYNTAX

```bash
monitorconfig reset-defaults [OPTIONS]
```

## DESCRIPTION

The `reset-defaults` command resets monitor settings to their factory default values. You can choose to reset all settings or only color-related settings.

This uses the following dxva2 functions internally:

- `RestoreMonitorFactoryDefaults` — resets all settings
- `RestoreMonitorFactoryColorDefaults` — resets only color settings

**Warning:** This will reset all or most monitor settings. Make sure to save any custom configurations beforehand if needed.

## OPTIONS

### `-d, --device <DEVICE>`

Specify the monitor by device name (e.g., `\\.\DISPLAY1`) or friendly name (e.g., "Dell U2723DE").

**Type:** String
**Required:** No (defaults to primary monitor)

### `-p, --primary`

Target the primary monitor.

**Type:** Flag
**Required:** No

### `-c, --color-only`

Reset only color-related settings instead of all settings.

**Type:** Flag
**Required:** No

## EXAMPLES

### Example 1: Reset all settings to factory defaults

```bash
monitorconfig reset-defaults --primary
```

Output:

```text
Monitor reset to factory defaults
```

### Example 2: Reset only color settings

```bash
monitorconfig reset-defaults --primary --color-only
```

Output:

```text
Monitor color settings reset to factory defaults
```

### Example 3: Reset specific monitor

```bash
monitorconfig reset-defaults --device "\\.\DISPLAY2"
```

### Example 4: Reset silently (for scripts)

```bash
monitorconfig reset-defaults --primary --silent
```

## NOTES

- If neither `--device` nor `--primary` is specified, the primary monitor is used by default.
- You can use either the device name (`\\.\DISPLAY1`) or the friendly name ("Dell U2723DE") with the `--device` option.
- Device name matching is case-insensitive.
- The monitor must support DDC/CI for this command to work.
- Use `save-settings` before resetting if you want to preserve a backup of your current configuration.
- Not all monitors support the color-only reset option. If `--color-only` fails, the monitor may only support a full factory reset.

## SEE ALSO

- [save-settings](save-settings.md) - Save current settings before resetting
- [set-brightness](set-brightness.md) - Set monitor brightness
- [set-contrast](set-contrast.md) - Set monitor contrast
- [set-vcp](set-vcp.md) - Set any VCP feature value
- [list](list.md) - List all monitors