# get-capabilities - Get Monitor Capabilities

## SYNOPSIS

Returns the DDC/CI capabilities string from the specified monitor.

## SYNTAX

```bash
monitorconfig get-capabilities [OPTIONS]
```

## DESCRIPTION

The `get-capabilities` command retrieves the monitor's capabilities string, which describes what features and VCP codes the monitor supports. This string is in a standardized format defined by the DDC/CI specification.

**Note:** Some manufacturers don't always fill out the capabilities string correctly, so it may not be 100% accurate. Use `scan-vcp` for a more reliable way to discover supported VCP codes.

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
monitorconfig get-capabilities --primary
```

Output (example):

```text
(prot(monitor)type(LCD)model(U2723DE)cmds(01 02 03 07 0C E3 F3)vcps(02 04 05 08 0B 0C 10 12 14 16 18 1A 52 60 62 AC AE B2 B6 C0 C6 C8 C9 CA CC(02 0A 01 02 03 04 05 06 08 09 0B 0C 0D 14 1E)D6(01 04 05)DC(00 02 03 05)DF E0 E1 E2(00 01 02 04 0E 12 14 19 0C 0D 0F 10 11 13)F0(0C)F1 F2)mswhql(1)mccs_ver(2.1))
```

### Example 2: Get capabilities of specific monitor

```bash
monitorconfig get-capabilities --device "\\.\DISPLAY2"
```

### Example 3: Get capabilities using friendly name

```bash
monitorconfig get-capabilities --device "Dell U2723DE"
```

### Example 4: Save capabilities to a file

```bash
monitorconfig get-capabilities --primary > capabilities.txt
```

## OUTPUT

The capabilities string is printed as raw text in the format defined by the MCCS (Monitor Control Command Set) specification. Key sections include:

- **prot**: Protocol supported (e.g., `monitor`)
- **type**: Display type (e.g., `LCD`)
- **model**: Monitor model name
- **cmds**: Supported DDC/CI commands
- **vcp**: Supported VCP codes, with allowed values in parentheses for non-continuous codes
- **mccs_ver**: MCCS version supported

## NOTES

- If neither `--device` nor `--primary` is specified, the primary monitor is used by default.
- You can use either the device name (`\\.\DISPLAY1`) or the friendly name ("Dell U2723DE") with the `--device` option.
- Device name matching is case-insensitive.
- The monitor must support DDC/CI for this command to work.
- The capabilities string is not always accurate — some manufacturers leave it incomplete or incorrect. Use `scan-vcp` to actively probe which VCP codes your monitor supports.
- This command does not support `--json` output; the raw capabilities string is always returned as plain text.

## SEE ALSO

- [scan-vcp](get-vcp.md) - Actively scan for supported VCP codes
- [get-vcp](get-vcp.md) - Get a specific VCP feature value
- [list](list.md) - List all monitors
