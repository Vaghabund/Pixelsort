#!/bin/bash
# Background update script - runs after app exits
# Args: $1 = project path, $2 = service name

PROJECT_PATH="$1"
SERVICE_NAME="$2"

echo "=========================================="
echo "Harpy Update - Background Rebuild"
echo "=========================================="

# Wait a moment for the app to fully exit
sleep 2

cd "$PROJECT_PATH" || exit 1

echo "Pulling latest code..."
git pull origin main 2>&1

if [ $? -ne 0 ]; then
    echo "ERROR: Git pull failed"
    exit 1
fi

echo "Rebuilding application (this may take 5-10 minutes)..."
cargo build --release 2>&1

if [ $? -ne 0 ]; then
    echo "ERROR: Build failed"
    exit 1
fi

echo "Build complete! Restarting service..."
sudo systemctl restart "$SERVICE_NAME"

echo "Update complete!"
