#!/bin/bash
# Install Harpy Pixel Sorter desktop shortcut

DESKTOP_FILE="Harpy.desktop"
SOURCE_PATH="$(dirname "$0")/$DESKTOP_FILE"
DEST_PATH="$HOME/Desktop/$DESKTOP_FILE"

# Copy to desktop
cp "$SOURCE_PATH" "$DEST_PATH"

# Make executable
chmod +x "$DEST_PATH"

# Mark as trusted (GNOME/Ubuntu)
if command -v gio &> /dev/null; then
    gio set "$DEST_PATH" metadata::trusted true
fi

echo "✅ Harpy desktop shortcut installed!"
echo "   Double-click 'Harpy Pixel Sorter' icon on desktop to launch"
