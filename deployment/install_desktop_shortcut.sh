#!/bin/bash
# Install Harpy desktop shortcuts

echo "Installing Harpy desktop shortcuts..."

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

# Install restart shortcut
RESTART_FILE="$DESKTOP_DIR/Harpy-Restart.desktop"
RESTART_SOURCE="$DEPLOY_DIR/Harpy-Restart.desktop"

cp "$RESTART_SOURCE" "$RESTART_FILE"
chmod +x "$RESTART_FILE"
gio set "$RESTART_FILE" metadata::trusted true 2>/dev/null || true

echo "✓ Desktop shortcuts installed!"
echo ""
echo "Desktop icons:"
echo "  • Harpy Pixel Sorter - Launch the app"
echo "  • Harpy-Restart - Restart the app service"
echo ""
