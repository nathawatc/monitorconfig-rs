# set-contrast / set-vcp - Set Monitor Color Settings

## SYNOPSIS

Adjust color and contrast settings on the monitor.

## SYNTAX

### Set contrast

```bash
monitor-config set-contrast <VALUE> [OPTIONS]
```

### Set individual color values via VCP

```bash
monitor-config set-vcp <CODE> <VALUE> [OPTIONS]
```

## DESCRIPTION

The Rust CLI provides color setting adjustment through multiple commands:

- `set-contrast` - Set contrast level (VCP code 0x12)
- `set-vcp` - Set individual color values using specific VCP codes

## COMMON COLOR VCP CODES

| Code | Name | Description | Typical Range |
|------|------|-------------|---------------|
| 0x12 | Contrast | Contrast level | 0-100 |
| 0x14 | Color Temperature | Color temperature | Varies by monitor |
| 0x16 | Red Video Gain | Red gain/tint | 0-100 |
| 0x18 | Green Video Gain | Green gain/tint | 0-100 |
| 0x1A | Blue Video Gain | Blue gain/tint | 0-100 |
| 0x6C | Red Black Level | Red drive/black level | 0-100 |
| 0x6E | Green Black Level | Green drive/black level | 0-100 |
| 0x70 | Blue Black Level | Blue drive/black level | 0-100 |
| 0x8A | Color Saturation | Overall color saturation | 0-100 |
| 0x5B | Gamma | Gamma adjustment | Varies |

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

Specify the monitor by device name or friendly name.

**Type:** String  
**Required:** No

### `-p, --primary`

Target the primary monitor.

**Type:** Flag  
**Required:** No

## EXAMPLES

### Example 1: Set contrast

```bash
monitor-config set-contrast 60 --primary
```

Output:

```text
Contrast set to 60
```

## PARAMETERS

### -BlueDrive

Adjusts the "drive" AKA "black" level for the blue color.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -BlueGain

Adjusts the "gain" AKA "tint" level for the blue color.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -BlueSaturation

Adjusts how saturated the blue color should be.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -ColorSaturation

Adjusts how saturated the overall image should be.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -ColorTemperature

Changes the color temperature on the display.

```yaml
Type: VCPMonitorColorTemperature
Parameter Sets: (All)
Aliases:
Accepted values: _None, _4000k, _5000k, _6500k, _7500k, _8200k, _9300k, _10000k, _11500k

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Contrast

Adjusts the contrast level on the display.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -CyanSaturation

Adjusts how saturated the cyan color should be.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Gamma

Adjusts the gamma level on the display.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -GreenDrive

Adjusts the "drive" AKA "black" level for the green color.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -GreenGain

Adjusts the "gain" AKA "tint" level for the green color.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -GreenSaturation

Adjusts how saturated the green color should be.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -MagentaSaturation

Adjusts how saturated the magenta color should be.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Monitor

The monitor(s) to get color info from.

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

### -RedDrive

Adjusts the "drive" AKA "black" level for the red color.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -RedGain

Adjusts the "gain" AKA "tint" level for the red color.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -RedSaturation

Adjusts how saturated the red color should be.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -YellowSaturation

Adjusts how saturated the yellow color should be.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -GrayScaleBlackExpansion

Expands the gray scale in the near black region, making darks appear more light or gray.  
Values between 0-3 can be used.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: False
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -GrayScaleWhiteExpansion

Expands the gray scale in the near white region, making whites appear more dark or gray.  
Values between 0-3 can be used.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

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
