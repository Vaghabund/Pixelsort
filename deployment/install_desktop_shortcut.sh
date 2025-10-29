#!/bin/bash
# Install Harpy desktop shortcut

echo "Installing Harpy desktop shortcut..."

DESKTOP_DIR="/home/pixelsort/Desktop"
DEPLOY_DIR="/home/pixelsort/Pixelsort/deployment"

# Create Desktop directory if it doesn't exist
mkdir -p "$DESKTOP_DIR"

# Install main Harpy app shortcut
HARPY_FILE="$DESKTOP_DIR/Harpy.desktop"
HARPY_SOURCE="$DEPLOY_DIR/Harpy.desktop"

cp "$HARPY_SOURCE" "$HARPY_FILE"
chmod +x "$HARPY_FILE"
gio set "$HARPY_FILE" metadata::trusted true 2>/dev/null || true

echo "✓ Harpy Pixel Sorter desktop shortcut installed!"
echo ""
echo "Double-click the 'Harpy Pixel Sorter' icon on your desktop to launch the app."
echo ""
