#!/bin/bash
# Install Harpy desktop shortcut for easy app restart

echo "Installing Harpy desktop shortcut..."

DESKTOP_DIR="/home/pixelsort/Desktop"
DESKTOP_FILE="$DESKTOP_DIR/Harpy-Restart.desktop"
SOURCE_FILE="/home/pixelsort/Pixelsort/deployment/Harpy-Restart.desktop"

# Create Desktop directory if it doesn't exist
mkdir -p "$DESKTOP_DIR"

# Copy desktop file
cp "$SOURCE_FILE" "$DESKTOP_FILE"

# Make it executable
chmod +x "$DESKTOP_FILE"

# Trust the desktop file (required for Ubuntu/Debian)
gio set "$DESKTOP_FILE" metadata::trusted true 2>/dev/null || true

echo "✓ Desktop shortcut installed!"
echo ""
echo "You can now double-click 'Harpy-Restart' on the desktop to restart the app."
echo ""
