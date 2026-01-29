#!/bin/bash
# Example: Night mode - reduce blue light by adjusting color gains

MODE=${1:-on}

if [ "$MODE" == "on" ]; then
    echo "Enabling night mode (reducing blue light)..."
    
    # Reduce blue gain, slightly reduce green
    monitor-config set-vcp 0x1A 40 --primary  # Blue gain
    monitor-config set-vcp 0x18 80 --primary  # Green gain
    monitor-config set-vcp 0x16 100 --primary # Red gain (full)
    
    echo "Night mode enabled"
elif [ "$MODE" == "off" ]; then
    echo "Disabling night mode (restoring color balance)..."
    
    # Restore default color gains
    monitor-config set-vcp 0x1A 100 --primary  # Blue gain
    monitor-config set-vcp 0x18 100 --primary  # Green gain
    monitor-config set-vcp 0x16 100 --primary  # Red gain
    
    echo "Night mode disabled"
else
    echo "Usage: $0 [on|off]"
    exit 1
fi
