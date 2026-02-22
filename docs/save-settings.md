# save-settings - Save Monitor Settings

## SYNOPSIS

Saves the current monitor settings to the monitor's non-volatile memory.

## SYNTAX

```bash
monitorconfig save-settings [OPTIONS]
```

## DESCRIPTION

The `save-settings` command instructs the monitor to save its current settings to non-volatile memory. This uses the dxva2 `SaveCurrentMonitorSettings` function internally.

After making adjustments to brightness, contrast, color, or other settings, use this command to ensure the changes persist after the monitor is powered off.

## OPTIONS

### `-d, --device <DEVICE>`

Specify the monitor by device name (e.g., `\\.\DISPLAY1`) or friendly name (e.g., "Dell U2723DE").

**Type:** String
**Required:** No (defaults to primary monitor)

### `-p, --primary`

Target the primary monitor.

**Type:** Flag
**Required:** No

## EXAMPLES

### Example 1: Save settings on primary monitor

```bash
monitorconfig save-settings --primary
```

Output:

```text
Monitor settings saved
```

### Example 2: Adjust settings and save

```bash
# Make adjustments
monitorconfig set-brightness 80 --primary
monitorconfig set-contrast 65 --primary

# Save the changes
monitorconfig save-settings --primary
```

### Example 3: Save settings on specific monitor

```bash
monitorconfig save-settings --device "\\.\DISPLAY2"
```

### Example 4: Save settings silently (for scripts)

```bash
monitorconfig save-settings --primary --silent
```

## NOTES

- If neither `--device` nor `--primary` is specified, the primary monitor is used by default.
- You can use either the device name (`\\.\DISPLAY1`) or the friendly name ("Dell U2723DE") with the `--device` option.
- Device name matching is case-insensitive.
- The monitor must support DDC/CI for this command to work.
- Not all monitors support saving settings via DDC/CI. If this command fails, the monitor may not implement the save function.
- Settings saved to monitor memory persist across power cycles. Use `reset-defaults` to revert to factory settings.

## SEE ALSO

- [reset-defaults](reset-defaults.md) - Reset monitor to factory defaults
- [set-brightness](set-brightness.md) - Set monitor brightness
- [set-contrast](set-contrast.md) - Set monitor contrast
- [set-vcp](set-vcp.md) - Set any VCP feature value
- [list](list.md) - List all monitors