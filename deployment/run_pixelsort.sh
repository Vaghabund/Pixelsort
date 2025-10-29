#!/bin/bash
# Raspberry Pi Pixel Sorter - Launcher script
# This script runs the app without blocking startup for updates

APP_DIR="/home/pixelsort/Pixelsort"

cd "$APP_DIR" || exit 1

echo "=========================================="
echo "Harpy Pixel Sorter - Starting..."
echo "=========================================="

# Ensure git ignores file permission changes (chmod +x)
git config core.fileMode false 2>/dev/null

echo "Starting Pixel Sorter..."
echo "Updates will be checked in background once app is running"
echo "=========================================="

BINARY="$APP_DIR/target/release/pixelsort-pi"

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "ERROR: Binary not found at $BINARY"
    echo "Please build manually: cargo build --release"
    exit 1
fi

# Run the application
"$BINARY"

# If app exits, wait a moment before this script ends
echo ""
echo "Application closed."
sleep 2
