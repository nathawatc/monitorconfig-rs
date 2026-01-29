# save-settings - Save Monitor Settings

## SYNOPSIS

Saves the current monitor settings to the monitor's non-volatile memory.

## SYNTAX

```bash
monitor-config save-settings [OPTIONS]
```

## DESCRIPTION

The `save-settings` command instructs the monitor to save its current settings to non-volatile memory. This uses VCP code 0x0C (Save Current Settings).

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
monitor-config save-settings --primary
```

Output:

```text
Monitor settings saved
```

### Example 2: Adjust settings and save

```bash
# Make adjustments
monitor-config set-brightness 80 --primary
monitor-config set-contrast 65 --primary

# Save the changes
monitor-config save-settings --primary
```

### Example 3: Save settings on specific monitor

```bash
monitor-config save-settings --device "\\.\DISPLAY2"
```

## PARAMETERS

### -Monitor

The monitor(s) where the modified settings should be saved.

```yaml
Type: VCPMonitor[]
Parameter Sets: (All)
Aliases:

Required: True
Position: Named
Default value: None
Accept pipeline input: True (ByValue)
Accept wildcard characters: False
```

### CommonParameters

This cmdlet supports the common parameters: -Debug, -ErrorAction, -ErrorVariable, -InformationAction, -InformationVariable, -OutVariable, -OutBuffer, -PipelineVariable, -Verbose, -WarningAction, and -WarningVariable. For more information, see [about_CommonParameters](http://go.microsoft.com/fwlink/?LinkID=113216).

## INPUTS

### MartinGC94.MonitorConfig.API.VCP.VCPMonitor[]

## OUTPUTS

### System.Object

## NOTES

## RELATED LINKS
