# set-vcp - Set VCP Feature Value

## SYNOPSIS

Sets the value of a specified VCP (VESA Command Protocol) feature code.

## SYNTAX

```bash
monitorconfig set-vcp <CODE> <VALUE> [OPTIONS]
```

## DESCRIPTION

The `set-vcp` command adjusts the value of any VCP feature code that the monitor supports. This provides low-level access to all monitor settings beyond the convenience commands like `set-brightness` and `set-contrast`.

## ARGUMENTS

### `<CODE>`

The VCP code to set (0-255, can be specified in hex like `0x10` or decimal like `16`).

**Type:** Unsigned byte (u8)  
**Required:** Yes

### `<VALUE>`

The value to set for the VCP code.

**Type:** Unsigned integer (u32)  
**Required:** Yes

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

### Example 1: Set brightness using VCP code

```bash
monitorconfig set-vcp 0x10 75 --primary
```

Output:

```text
VCP code 0x10 set to 75
```

### Example 2: Change input source

```bash
# Set input source to HDMI (value may vary by monitor)
monitorconfig set-vcp 0x60 3 --device "\\.\DISPLAY1"
```

### Example 3: Turn off monitor

```bash
# VCP code 0xD6 (Power Mode): 1=On, 4=Off
monitorconfig set-vcp 0xD6 4 --primary
```

### Example 4: Set color temperature

```bash
# VCP code 0x14 (Color Temperature)
monitorconfig set-vcp 0x14 6500 --primary
```

## PARAMETERS

### -Monitor

The monitor(s) where the VCP code + value should be set.

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

The VCP code to adjust. You can get a list of the supported VCP codes on the monitor with `Get-MonitorVCPResponse -All`.

```yaml
Type: Byte
Parameter Sets: (All)
Aliases:

Required: True
Position: 0
Default value: None
Accept pipeline input: False
Accept wildcard characters: False
```

### -Value

The value to set. Read the DDC/CI documentation to find out what the value means for your specific VCP code.
Alternatively, you can experiment with different values and note down the results.

```yaml
Type: UInt32
Parameter Sets: (All)
Aliases:

Required: True
Position: 1
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
