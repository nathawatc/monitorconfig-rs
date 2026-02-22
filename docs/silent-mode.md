# Silent Mode and Background Tasks

## Overview

The `--silent` (or `-s`) flag suppresses all console output, making MonitorConfig ideal for
background automation, Task Scheduler jobs, and startup scripts.

Two additional global flags — `--wait` and `--retry` (on `set-vcp`) — make automated execution
reliable in environments where the monitor's DDC/CI firmware may not be ready immediately, such
as after resume from sleep or hibernate.

## Global Flags for Automation

| Flag               | Short | Applies To   | Description                        |
| ------------------ | ----- | ------------ | ---------------------------------- |
| `--silent`         | `-s`  | All commands | Suppress all stdout output         |
| `--wait <SECONDS>` | `-w`  | All commands | Sleep before executing (see below) |

## Usage

### Silent Mode

Add `--silent` or `-s` as a global flag to any command:

```bash
monitorconfig set-brightness 50 --primary --silent
monitorconfig set-vcp 0x14 11 -d "Lenovo L22i-40" -s
monitorconfig get-brightness --primary --silent
```

### Resume-safe Execution (`--wait` and `--retry`)

After a system resumes from sleep or hibernate, the monitor's internal DDC/CI controller can
take 10–15 seconds after the display signal is restored before it is ready to accept I²C
commands. Sending a `SetVCPFeature` command during that window can corrupt the monitor's MCU
state, causing a **black screen that requires a full power cycle to recover**.

Use `--wait` to delay command execution inside the process and `--retry` / `--retry-delay` on
`set-vcp` to automatically re-attempt if the monitor is still not ready:

```bash
monitorconfig set-vcp 0x14 11 -d "Lenovo L22i-40" -s --wait 10 --retry 3 --retry-delay 8
```

This command will:

1. Start immediately when Task Scheduler launches it
2. Sleep 10 seconds (inside the process, after the display driver is already up)
3. Attempt `SetVCPFeature(0x14, 11)`
4. If that fails, wait 8 seconds and retry — up to 3 more times
5. Exit with code `0` on success or `1` if all attempts fail

> **Why `--wait` beats the Task Scheduler trigger `<Delay>`**
>
> The trigger `<Delay>` counts from when Windows fires the resume event, which can happen while
> the GPU is still negotiating EDID with the monitor. When the delay expires, the display driver
> reports the monitor as present — but the monitor's internal MCU may need additional time beyond
> that before it accepts DDC/CI commands. `--wait` runs _inside_ the already-launched process,
> adding time on top of however long driver initialization took, making it a reliable buffer
> regardless of resume speed.

## When to Use Each Flag

| Scenario                                          | Recommended Flags                        |
| ------------------------------------------------- | ---------------------------------------- |
| Task Scheduler, no automation concerns            | `-s`                                     |
| Task Scheduler on startup / user logon            | `-s --wait 5`                            |
| Task Scheduler triggered by resume from sleep     | `-s --wait 10 --retry 3 --retry-delay 8` |
| Task Scheduler triggered by resume from hibernate | `-s --wait 15 --retry 3 --retry-delay 8` |
| Interactive scripts / batch files                 | `-s` (optional)                          |

## Console Window Flash Prevention

### Problem

Even with `--silent`, Windows may briefly show a console window when launching the executable
from Task Scheduler or other GUI contexts.

### Solution 1: Windows GUI Subsystem Build

Build MonitorConfig without console window support using Cargo features:

```bash
# Windows build
cargo build --release --features gui-subsystem

# Cross-compile from Linux
cargo build --release --target x86_64-pc-windows-gnu --features gui-subsystem
```

> **Note**: With a GUI subsystem build there is no console output at all, including errors.
> Only use this for production automation where errors are logged or checked via exit code
> elsewhere. For development and testing, use the standard build.

### Solution 2: VBScript Wrapper

Create `run_silent.vbs`:

```vbscript
Set WshShell = CreateObject("Wscript.Shell")
WshShell.Run """C:\Program Files\NChalapinyo\monitorconfig.exe"" set-vcp 0x14 11 -d ""Lenovo L22i-40"" -s --wait 10 --retry 3 --retry-delay 8", 0, True
```

Then use the `.vbs` file as the Task Scheduler action instead of the `.exe` directly.

### Solution 3: PowerShell Hidden Window

```powershell
$proc = Start-Process `
    -FilePath "C:\Program Files\NChalapinyo\monitorconfig.exe" `
    -ArgumentList "set-vcp 0x14 11 -d 'Lenovo L22i-40' -s --wait 10 --retry 3 --retry-delay 8" `
    -WindowStyle Hidden `
    -Wait `
    -PassThru

exit $proc.ExitCode
```

## Task Scheduler Setup

### Action

| Field          | Value                                                                        |
| -------------- | ---------------------------------------------------------------------------- |
| Program/script | `"C:\Program Files\NChalapinyo\monitorconfig.exe"`                           |
| Add arguments  | `set-vcp -d "Lenovo L22i-40" 0x14 11 -s --wait 10 --retry 3 --retry-delay 8` |
| Start in       | `C:\Program Files\NChalapinyo\`                                              |

### Settings

| Setting                        | Recommended Value | Reason                                                            |
| ------------------------------ | ----------------- | ----------------------------------------------------------------- |
| `<MultipleInstancesPolicy>`    | `IgnoreNew`       | Prevents duplicate runs when multiple resume events fire together |
| `<ExecutionTimeLimit>`         | `PT2M`            | Covers `--wait 10` + up to 3 retries × 8 s ≈ 50 s worst case      |
| `<DisallowStartIfOnBatteries>` | `true`            | No external monitor when on battery                               |
| `<StopIfGoingOnBatteries>`     | `true`            | Abort if power is pulled mid-run                                  |

### Trigger Delay

Set the trigger `<Delay>` to `PT2S` — just enough time for the process to be schedulable.
The real waiting is handled by `--wait` inside the process, which is more reliable than a
fixed scheduler delay.

### Trigger Events (Resume-from-sleep pattern)

```xml
<QueryList>
  <Query Id="0" Path="System">
    <!-- Resume from sleep -->
    <Select Path="System">
      *[System[Provider[@Name='Microsoft-Windows-Power-Troubleshooter']
        and (Level=4 or Level=0) and (EventID=1)]]
    </Select>
    <!-- Connected standby exit -->
    <Select Path="System">
      *[System[Provider[@Name='Microsoft-Windows-Kernel-Power'] and (EventID=507)]]
    </Select>
    <!-- User logon -->
    <Select Path="System">
      *[System[Provider[@Name='Microsoft-Windows-Winlogon'] and (EventID=7001)]]
    </Select>
  </Query>
</QueryList>
```

## Exit Codes

| Code | Meaning                                                    |
| ---- | ---------------------------------------------------------- |
| `0`  | Success                                                    |
| `1`  | Error (written to stderr unless using GUI subsystem build) |

In silent mode, errors are still written to stderr. In a GUI subsystem build, nothing is written
anywhere — check the exit code.

## Error Handling in Scripts

### Batch File

```batch
@echo off
"C:\Program Files\NChalapinyo\monitorconfig.exe" set-vcp 0x14 11 -d "Lenovo L22i-40" -s --wait 10 --retry 3 --retry-delay 8
if %ERRORLEVEL% NEQ 0 (
    echo %DATE% %TIME% - Error applying VCP preset >> C:\logs\monitorconfig.log
)
```

### PowerShell

```powershell
& "C:\Program Files\NChalapinyo\monitorconfig.exe" set-vcp 0x14 11 -d "Lenovo L22i-40" -s --wait 10 --retry 3 --retry-delay 8
if ($LASTEXITCODE -ne 0) {
    Write-EventLog -LogName Application -Source "MonitorConfig" `
        -EventId 1001 -EntryType Error `
        -Message "Failed to apply VCP preset after retries"
}
```

## Best Practices

1. **Always use `--silent`** with Task Scheduler to avoid unnecessary output.
2. **Test commands manually first** without `--silent` to verify they work interactively.
3. **Use absolute paths** in Task Scheduler for reliability.
4. **Use `--wait` instead of a large trigger delay** for resume-triggered tasks — it is more
   reliable because it runs after the display driver is already up.
5. **If commands work interactively but fail after resume**, increase `--wait` in 5-second
   increments until the task is consistently reliable.
6. **Log errors to a file** when running with a GUI subsystem build, since stderr is suppressed.
7. **Use `--retry 3 --retry-delay 8`** as a sensible default for any resume-triggered task.

## Comparison of Console Suppression Methods

| Method              | Window Flash     | See Errors   | Requires Rebuild |
| ------------------- | ---------------- | ------------ | ---------------- |
| `--silent` flag     | Yes (brief)      | Yes (stderr) | No               |
| VBScript wrapper    | No               | No           | No               |
| PowerShell Hidden   | Yes (very brief) | Yes          | No               |
| GUI subsystem build | No               | No           | Yes              |

## Examples

### Re-apply Color Preset After Resume (BrightStay pattern)

Some monitors reset VCP settings after waking from sleep. Trigger on resume events and use
`--wait` + `--retry` to reliably re-apply once the monitor is ready:

```batch
monitorconfig.exe set-vcp 0x14 11 -d "Lenovo L22i-40" -s --wait 10 --retry 3 --retry-delay 8
```

### Morning / Evening Brightness Automation

```batch
REM morning_bright.bat
monitorconfig.exe set-brightness 100 --primary --silent
monitorconfig.exe set-vcp 0x14 11 --primary --silent

REM evening_dim.bat
monitorconfig.exe set-brightness 30 --primary --silent
monitorconfig.exe set-vcp 0x14 3 --primary --silent
```

### Multi-Monitor Setup

```batch
REM setup_monitors.bat
monitorconfig.exe set-brightness 80 -d "\\.\DISPLAY1" --silent
monitorconfig.exe set-brightness 80 -d "\\.\DISPLAY2" --silent
monitorconfig.exe set-contrast 75 -d "\\.\DISPLAY1" --silent
monitorconfig.exe set-contrast 75 -d "\\.\DISPLAY2" --silent
```

## Related

- [set-vcp](set-vcp.md) — Full reference for `--wait`, `--retry`, and `--retry-delay`
- [Building](building.md) — How to build with the `gui-subsystem` feature
