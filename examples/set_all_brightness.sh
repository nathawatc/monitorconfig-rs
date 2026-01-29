#!/bin/bash
# Example: Set brightness across all monitors

BRIGHTNESS=${1:-75}

echo "Setting all monitors to brightness: $BRIGHTNESS"
echo ""

# Get all monitor device names
MONITORS=$(monitor-config list --json | grep -o '\\\\.\\DISPLAY[0-9]*' | sort -u)

for monitor in $MONITORS; do
    echo "Setting $monitor to $BRIGHTNESS..."
    monitor-config set-brightness $BRIGHTNESS --device "$monitor"
done

echo ""
echo "Done! All monitors set to brightness $BRIGHTNESS"
