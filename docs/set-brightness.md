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

## PARAMETERS

### -ALSBrightness

Set the ambient light sensor brightness value.  
If brightness has been manually adjusted, the brightness policy has to be reverted by using the RevertToPolicy parameter.

```yaml
Type: Byte
Parameter Sets: WMIOptions
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -ALSEnabled

Controls whether or not brightness should be adjusted by the Ambient Light Sensor.
If brightness has been manually adjusted, the brightness policy has to be reverted by using the RevertToPolicy parameter.

```yaml
Type: Boolean
Parameter Sets: WMIOptions
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Monitor

The monitor to adjust brightness settings for.

```yaml
Type: Monitor[]
Parameter Sets: (All)
Aliases:

Required: True
Position: Named
Default value: None
Accept pipeline input: True (ByValue)
Accept wildcard characters: False
```

### -RevertToPolicy

Reverts the brightness policy to system managed if manual brightness values have previously been used.

```yaml
Type: SwitchParameter
Parameter Sets: WMIOptions
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Timeout

Sets the timeout for changing the brightness on WMIMonitors. It's rarely needed.

```yaml
Type: UInt32
Parameter Sets: WMIOptions
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Value

The brightness value to set.

```yaml
Type: UInt32
Parameter Sets: Default
Aliases:

Required: True
Position: 0
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

```yaml
Type: UInt32
Parameter Sets: WMIOptions
Aliases:

Required: False
Position: 0
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### CommonParameters

This cmdlet supports the common parameters: -Debug, -ErrorAction, -ErrorVariable, -InformationAction, -InformationVariable, -OutVariable, -OutBuffer, -PipelineVariable, -Verbose, -WarningAction, and -WarningVariable. For more information, see [about_CommonParameters](http://go.microsoft.com/fwlink/?LinkID=113216).

## INPUTS

### MartinGC94.MonitorConfig.API.Monitor[]

## OUTPUTS

### System.Object

## NOTES

## RELATED LINKS
