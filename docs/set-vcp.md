# set-vcp - Set VCP Feature Value

## SYNOPSIS

Sets the value of a specified VCP (VESA Command Protocol) feature code.

## SYNTAX

```bash
monitorconfig set-vcp <CODE> <VALUE> [OPTIONS]
```

## DESCRIPTION

The `set-vcp` command adjusts the value of any VCP feature code that the monitor supports. This provides low-level access to all monitor settings beyond the convenience commands like `set-brightness` and `set-contrast`.

Includes built-in `--wait` and `--retry` options for reliable execution in automated environments such as Windows Task Scheduler, where the monitor's DDC/CI firmware may not yet be ready immediately after resume from sleep or hibernate.

## ARGUMENTS

### `<CODE>`

The VCP code to set (0–255). Can be specified in hex (`0x10`) or decimal (`16`).

**Type:** Unsigned byte (u8)
**Required:** Yes

### `<VALUE>`

The value to set for the VCP code.

**Type:** Unsigned integer (u32)
**Required:** Yes

## OPTIONS

### `-d, --device <DEVICE>`

Specify the monitor by device name (e.g., `\\.\DISPLAY1`) or friendly name (e.g., `"Dell U2723DE"`).

**Type:** String
**Required:** No (defaults to primary monitor)

### `-p, --primary`

Target the primary monitor.

**Type:** Flag
**Required:** No

### `-w, --wait <SECONDS>`

Sleep this many seconds **before** acquiring any monitor handle or sending any DDC/CI command.

This is the recommended way to handle post-resume timing in Task Scheduler. Unlike the Task Scheduler trigger `<Delay>`, which counts from when the _resume event fires_, `--wait` runs inside the process after the display driver is already up — giving the monitor's internal DDC/CI firmware the additional time it needs to finish its own power-on initialization sequence.

Sending a `SetVCPFeature` command to a monitor whose MCU is still initializing can corrupt its state and cause a black screen that requires a full power cycle to recover.

**Type:** Integer (seconds)
**Required:** No
**Default:** `0`

### `--retry <COUNT>`

Retry the command up to COUNT additional times if it fails. On each retry the monitor handle is fully re-enumerated from scratch, so a transient failure (e.g. monitor not yet detected by the driver) is recovered automatically.

**Type:** Integer
**Required:** No
**Default:** `0`

### `--retry-delay <SECONDS>`

Seconds to wait between retry attempts. Gives the monitor's DDC/CI controller time to finish initializing before the next attempt.

**Type:** Integer (seconds)
**Required:** No
**Default:** `5`

### `-s, --silent`

Suppress all console output. Errors are still written to stderr unless the binary was built with the `gui-subsystem` feature.

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

### Example 4: Set color temperature preset

```bash
# VCP code 0x14 (Color Temperature Preset)
monitorconfig set-vcp 0x14 11 --primary
```

### Example 5: Resume-safe execution for Task Scheduler

Some monitors (e.g. Lenovo L22i-40) reset certain VCP settings after waking from sleep. The
following command waits 10 seconds after launch (giving the monitor's DDC/CI firmware time to
initialize), then attempts the command up to 4 times total (1 + 3 retries) with 8-second gaps
between attempts:

```bash
monitorconfig set-vcp 0x14 11 -d "Lenovo L22i-40" -s --wait 10 --retry 3 --retry-delay 8
```

**Recommended Task Scheduler settings when using this pattern:**

| Setting                     | Value       | Reason                                                         |
| --------------------------- | ----------- | -------------------------------------------------------------- |
| Trigger `<Delay>`           | `PT2S`      | Minimal — real delay is handled by `--wait` inside the process |
| `<ExecutionTimeLimit>`      | `PT2M`      | Covers `--wait 10` + up to 3 retries × 8 s ≈ 50 s worst case   |
| `<MultipleInstancesPolicy>` | `IgnoreNew` | Prevents duplicate runs if multiple resume events fire at once |

> **Why `--wait` beats the Task Scheduler trigger delay**
>
> The trigger `<Delay>` counts from when Windows fires the resume event, which can happen while
> the GPU is still negotiating EDID with the monitor. By the time the delay expires, the display
> driver reports the monitor as present — but the monitor's internal MCU may need a further
> 10–15 seconds before it accepts I²C/DDC commands. `--wait` runs _inside_ the already-launched
> process, so it adds time on top of however long driver initialization took, making it a
> reliable buffer regardless of how fast the system resumed.

## NOTES

- VCP code support varies by monitor model. Use `scan-vcp` to discover which codes your monitor supports.
- Always test commands manually (without `--silent`) before deploying to Task Scheduler.
- If a command works interactively but fails after resume, increase `--wait` in 5-second increments until reliable.
- DDC/CI must be enabled in the monitor's OSD (On-Screen Display) settings.

## RELATED LINKS

- [get-vcp](get-vcp.md) — Read a VCP feature value
- [scan-vcp](../README.md#scan-monitor-for-all-supported-vcp-codes) — Discover supported VCP codes
- [silent-mode](silent-mode.md) — Background task and Task Scheduler guidance
