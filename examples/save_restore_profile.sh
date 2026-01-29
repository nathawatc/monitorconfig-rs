#!/bin/bash
# Example: Save monitor profile to JSON and restore later

PROFILE_FILE="${1:-monitor_profile.json}"

if [ "$2" == "restore" ]; then
    echo "Restoring monitor profile from $PROFILE_FILE..."
    
    if [ ! -f "$PROFILE_FILE" ]; then
        echo "Error: Profile file not found: $PROFILE_FILE"
        exit 1
    fi
    
    # Parse JSON and restore settings
    # This is a simplified example - real implementation would parse JSON properly
    cat "$PROFILE_FILE"
    
    echo ""
    echo "Profile restored (manual adjustment needed for full restore)"
else
    echo "Saving monitor profile to $PROFILE_FILE..."
    
    # Save all monitor information
    {
        echo "{"
        echo "  \"timestamp\": \"$(date -Iseconds)\","
        echo "  \"monitors\": $(monitor-config list --json),"
        echo "  \"brightness\": {"
        
        FIRST=1
        for display in $(monitor-config list --json | grep -o '\\\\.\\DISPLAY[0-9]*' | sort -u); do
            if [ $FIRST -eq 0 ]; then
                echo ","
            fi
            FIRST=0
            
            BRIGHTNESS=$(monitor-config get-brightness --device "$display" --json)
            echo -n "    \"$display\": $BRIGHTNESS"
        done
        
        echo ""
        echo "  }"
        echo "}"
    } > "$PROFILE_FILE"
    
    echo "Profile saved to $PROFILE_FILE"
fi
