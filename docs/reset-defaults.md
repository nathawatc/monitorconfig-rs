# reset-defaults - Reset Monitor to Factory Defaults

## SYNOPSIS

Resets monitor settings to factory default values.

## SYNTAX

```bash
monitorconfig reset-defaults [OPTIONS]
```

## DESCRIPTION

The `reset-defaults` command resets monitor settings to their factory default values. You can choose to reset all settings or only color-related settings.

This uses VCP codes:

- 0x04 - Restore Factory Defaults (all settings)
- 0x08 - Restore Factory Color Defaults (color only)

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

## PARAMETERS

### -Confirm

Prompts you for confirmation before running the cmdlet.

```yaml
Type: SwitchParameter
Parameter Sets: (All)
Aliases: cf

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Kind

The settings that should be reset.

```yaml
Type: ResetKind
Parameter Sets: (All)
Aliases:
Accepted values: All, BrightnessAndContrast, Geometry, Colors, TVDefaults

Required: True
Position: 0
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Monitor

The monitor(s) to reset settings on.

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

### -WhatIf

Shows what would happen if the cmdlet runs.
The cmdlet is not run.

```yaml
Type: SwitchParameter
Parameter Sets: (All)
Aliases: wi

Required: False
Position: Named
Default value: None
Accept pipeline input: False
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
