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
**Required:** No

### `-p, --primary`

Target the primary monitor.

**Type:** Flag  
**Required:** No

### `-j, --json`

Output in JSON format.

**Type:** Flag  
**Required:** No

## EXAMPLES

### Example 1: Get contrast

```bash
monitorconfig get-contrast --primary
```

Output:

```text
Current contrast: 50 (min: 0, max: 100)
```

## PARAMETERS

### -Monitor

The monitor(s) to return color details for.

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

### MartinGC94.MonitorConfig.API.MonitorColorInfo

## NOTES

## RELATED LINKS
