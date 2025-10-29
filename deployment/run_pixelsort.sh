#!/bin/bash
# Raspberry Pi Pixel Sorter - Auto-update launcher script
# This script updates the app from git and runs it

APP_DIR="/home/pixelsort/Pixelsort"
REPO_URL="https://github.com/Vaghabund/Pixelsort.git"

cd "$APP_DIR" || exit 1

echo "=========================================="
echo "Harpy Pixel Sorter - Starting..."
echo "=========================================="

# Ensure git ignores file permission changes (chmod +x)
git config core.fileMode false 2>/dev/null

# Check for internet connectivity
if ping -c 1 -W 2 github.com &> /dev/null; then
    echo "Internet connected. Checking for updates..."
    
    # Save current HEAD
    LOCAL_BEFORE=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
    
    # Fetch latest changes
    git fetch origin main 2>/dev/null
    
    # Check if we're behind
    LOCAL=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
    REMOTE=$(git rev-parse origin/main 2>/dev/null || echo "unknown")
    
    if [ "$LOCAL" != "$REMOTE" ] && [ "$REMOTE" != "unknown" ]; then
        echo "Updates found! Updating..."
        
        # Stash any local changes (like file permissions)
        git stash --include-untracked &> /dev/null
        
        # Pull changes
        if git pull origin main; then
            echo "✓ Update successful!"
            
            # Make scripts executable (in case permissions lost)
            chmod +x "$APP_DIR/deployment/run_pixelsort.sh"
            chmod +x "$APP_DIR/deployment/setup_autostart.sh"
            
            # Get new HEAD after pull
            LOCAL_AFTER=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
            
            # Check if source code changed between before and after
            if git diff --name-only "$LOCAL_BEFORE" "$LOCAL_AFTER" | grep -q "^src/\|Cargo.toml"; then
                echo "Source code changed. Rebuilding..."
                NEEDS_REBUILD=true
            else
                echo "No source changes. Using existing binary."
                NEEDS_REBUILD=false
            fi
        else
            echo "⚠ Update failed. Using current version."
            NEEDS_REBUILD=false
        fi
    else
        echo "Already up to date."
        NEEDS_REBUILD=false
    fi
else
    echo "No internet connection. Skipping update check."
    NEEDS_REBUILD=false
fi

echo "Starting Pixel Sorter..."
echo "=========================================="

BINARY="$APP_DIR/target/release/pixelsort-pi"

# Rebuild if needed
if [ "$NEEDS_REBUILD" = true ]; then
    if command -v cargo &> /dev/null; then
        echo "Building updated code..."
        if cargo build --release; then
            echo "✓ Build successful!"
        else
            echo "✗ Build failed! Using old binary if available."
        fi
    else
        echo "⚠ Cargo not available. Cannot rebuild."
    fi
fi

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
