# get-capabilities - Get Monitor Capabilities

## SYNOPSIS

Returns the DDC/CI capabilities string from the specified monitor.

## SYNTAX

```bash
monitor-config get-capabilities [OPTIONS]
```

## DESCRIPTION

The `get-capabilities` command retrieves the monitor's capabilities string, which describes what features and VCP codes the monitor supports. This string is in a standardized format defined by the DDC/CI specification.

**Note:** Some manufacturers don't always fill out the capabilities string correctly, so it may not be 100% accurate.

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

### Example 1: Get capabilities of primary monitor

```bash
monitor-config get-capabilities --primary
```

Output (example):

```text
(prot(monitor)type(LCD)model(U2723DE)cmds(01 02 03 07 0C E3 F3)vcps(02 04 05 08 0B 0C 10 12 14 16 18 1A 52 60 62 AC AE B2 B6 C0 C6 C8 C9 CA CC(02 0A 01 02 03 04 05 06 08 09 0B 0C 0D 14 1E)D6(01 04 05)DC(00 02 03 05)DF E0 E1 E2(00 01 02 04 0E 12 14 19 0C 0D 0F 10 11 13)F0(0C)F1 F2)mswhql(1)mccs_ver(2.1))
```

## PARAMETERS

### -Monitor

The monitor(s) to return details for.

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

### MartinGC94.MonitorConfig.API.MonitorDetails

## NOTES

## RELATED LINKS
