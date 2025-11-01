#!/bin/bash
# Background update script - runs after app exits
# Args: $1 = project path, $2 = service name

PROJECT_PATH="$1"
SERVICE_NAME="$2"

echo "=========================================="
echo "Harpy Update - Background Rebuild"
echo "Date: $(date)"
echo "=========================================="

# Wait a moment for the app to fully exit
echo "Waiting for app to exit..."
sleep 3

cd "$PROJECT_PATH" || {
    echo "ERROR: Failed to change to directory $PROJECT_PATH"
    exit 1
}

echo "Pulling latest code..."
git pull origin main

if [ $? -ne 0 ]; then
    echo "ERROR: Git pull failed - restarting service with existing binary"
    sudo systemctl restart "$SERVICE_NAME"
    exit 1
fi

echo "Rebuilding application (this may take 5-10 minutes)..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "ERROR: Build failed - restarting service with existing binary"
    sudo systemctl restart "$SERVICE_NAME"
    exit 1
fi

echo "Build complete! Restarting service..."
sudo systemctl restart "$SERVICE_NAME"

echo "Update complete at $(date)"
echo "=========================================="
