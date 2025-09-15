#!/bin/bash
# Script to run on Raspberry Pi for easy development
# Usage: ./pi_dev.sh [command]

cd /home/pixelsort/Pixelsort
source ~/.cargo/env

case "$1" in
    "pull")
        echo "Pulling latest changes..."
        git pull origin main
        ;;
    "build")
        echo "Building project..."
        cargo build --release
        ;;
    "run")
        echo "Running pixelsort..."
        echo "Make sure your display is connected!"
        LIBGL_ALWAYS_SOFTWARE=1 DISPLAY=:0 ./target/release/pixelsort
        ;;
    "dev")
        echo "Full development cycle: pull -> build -> run"
        git pull origin main
        cargo build --release
        if [ $? -eq 0 ]; then
            echo "Starting pixelsort..."
            LIBGL_ALWAYS_SOFTWARE=1 DISPLAY=:0 ./target/release/pixelsort
        else
            echo "Build failed!"
        fi
        ;;
    "logs")
        echo "Showing recent logs..."
        tail -f ~/.local/share/nannou/pixelsort/logs/* 2>/dev/null || echo "No logs found"
        ;;
    *)
        echo "Usage: ./pi_dev.sh [pull|build|run|dev|logs]"
        echo "  pull  - Pull latest changes from GitHub"
        echo "  build - Build the project"  
        echo "  run   - Run the compiled program"
        echo "  dev   - Pull, build, and run in one command"
        echo "  logs  - Show application logs"
        ;;
esac
