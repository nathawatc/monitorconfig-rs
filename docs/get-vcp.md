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
VCP Code: 0x10 - Brightness
Description: Luminance of the image
Current value: 75 (max: 100)
Type: Continuous
```

### Example 2: Get input source

```bash
monitorconfig get-vcp 0x60 --device "\\.\DISPLAY1"
```

### Example 3: Scan all supported VCP codes

```bash
monitorconfig scan-vcp --primary
```

Output:

```text
Scanning monitor for supported VCP codes...
Found 15 supported VCP codes

Code   Name                                CurrentValue MaxValue Description
------------------------------------------------------------------------------------------------------------------------
0x10   Brightness                          75           100      Luminance of the image
0x12   Contrast                            50           100      Contrast of the image
0x14   Color Temperature                   6500         10000    Select color temperature
0x60   Input Source                        3            4        Select input source
...
```

## PARAMETERS

### -All

Queries the monitor for all possible VCP codes from 0-255 and returns the ones that are supported, along with their value.

```yaml
Type: SwitchParameter
Parameter Sets: All
Aliases:

Required: True
Position: Named
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Monitor

The monitor(s) to return VCP code details for.

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

### -VCPCode

The VCP codes (0-255) to query.

```yaml
Type: Byte[]
Parameter Sets: SpecifiedCode
Aliases:

Required: True
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

### MartinGC94.MonitorConfig.API.VCP.VCPFeatureResponse

## NOTES

## RELATED LINKS
