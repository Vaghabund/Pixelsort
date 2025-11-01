#!/bin/bash
# Wait for X11 to be ready before starting the application
# This script polls for X11 availability instead of using a fixed delay

MAX_WAIT=30  # Maximum seconds to wait
RETRY_INTERVAL=0.2  # Check every 200ms

echo "Waiting for X11 display server to be ready..."

elapsed=0
while [ $(echo "$elapsed < $MAX_WAIT" | bc) -eq 1 ]; do
    # Try to connect to the X display
    if DISPLAY=:0 xset q &>/dev/null; then
        echo "✓ X11 is ready (took ${elapsed}s)"
        exit 0
    fi
    
    sleep $RETRY_INTERVAL
    elapsed=$(echo "$elapsed + $RETRY_INTERVAL" | bc)
done

echo "⚠ Timeout waiting for X11 after ${MAX_WAIT}s, attempting to start anyway..."
exit 0
