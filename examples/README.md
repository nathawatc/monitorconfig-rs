# MonitorConfig Examples

This directory contains example scripts demonstrating common use cases for the MonitorConfig CLI tool.

## Available Examples

### 1. Set All Brightness (`set_all_brightness.sh` / `.bat`)

Set the same brightness level across all connected monitors.

**Usage:**
```bash
# Linux/macOS
./examples/set_all_brightness.sh 75

# Windows
examples\set_all_brightness.bat 75
```

### 2. Save and Restore Profile (`save_restore_profile.sh`)

Save current monitor settings to a JSON file and restore them later.

**Usage:**
```bash
# Save current profile
./examples/save_restore_profile.sh my_profile.json

# Restore profile
./examples/save_restore_profile.sh my_profile.json restore
```

### 3. Night Mode (`night_mode.sh`)

Enable/disable night mode by adjusting monitor color gains to reduce blue light.

**Usage:**
```bash
# Enable night mode
./examples/night_mode.sh on

# Disable night mode
./examples/night_mode.sh off
```

## Custom Scripts

You can create your own automation scripts using the monitor-config CLI. Here are some ideas:

### Time-Based Brightness Adjustment

```bash
#!/bin/bash
# Adjust brightness based on time of day

HOUR=$(date +%H)

if [ $HOUR -ge 6 ] && [ $HOUR -lt 12 ]; then
    # Morning: Medium brightness
    monitor-config set-brightness 70 --primary
elif [ $HOUR -ge 12 ] && [ $HOUR -lt 18 ]; then
    # Afternoon: High brightness
    monitor-config set-brightness 90 --primary
else
    # Evening/Night: Low brightness
    monitor-config set-brightness 40 --primary
fi
```

### Monitor Switching

```bash
#!/bin/bash
# Switch input source on monitor

# DisplayPort (typically value 15)
monitor-config set-vcp 0x60 15 --primary

# HDMI (typically value 17)
# monitor-config set-vcp 0x60 17 --primary
```

### Power Management

```bash
#!/bin/bash
# Turn monitors on/off

# Turn on
monitor-config set-vcp 0xD6 1 --primary

# Turn off
# monitor-config set-vcp 0xD6 4 --primary
```

## Integration with Task Scheduler (Windows)

You can schedule these scripts using Windows Task Scheduler:

1. Open Task Scheduler
2. Create New Task
3. Set trigger (e.g., daily at specific time)
4. Set action: `monitor-config.exe set-brightness 50 --primary`
5. Save and enable

## Integration with cron (Linux)

```bash
# Edit crontab
crontab -e

# Add entries:
# Set brightness at 8 AM
0 8 * * * /path/to/monitor-config set-brightness 90 --primary

# Set brightness at 8 PM
0 20 * * * /path/to/monitor-config set-brightness 40 --primary
```

## JSON Output Processing

Many commands support `--json` output for easy parsing:

```bash
# Using jq to parse JSON
monitor-config list --json | jq '.[] | select(.is_primary == true) | .device_name'

# Using Python
monitor-config get-brightness --primary --json | python -c "import sys, json; data = json.load(sys.stdin); print(f'Current: {data[\"current\"]}')"
```

## Error Handling

Example with error handling:

```bash
#!/bin/bash

set -e  # Exit on error

if ! monitor-config set-brightness 75 --primary 2>/dev/null; then
    echo "Warning: Could not set brightness on primary monitor"
    echo "Trying alternative method..."
    monitor-config set-vcp 0x10 75 --primary
fi
```

## Tips

1. **Test First**: Always test commands manually before automating
2. **Check Capabilities**: Use `get-capabilities` to see what your monitor supports
3. **Use --json**: For scripting, always use `--json` flag for reliable parsing
4. **Error Codes**: The CLI returns non-zero exit codes on errors
5. **VCP Codes**: Different monitors may use different values for the same VCP code (e.g., input source)
