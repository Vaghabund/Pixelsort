# Harpy Pixel Sorter

Touch-optimized pixel sorting handheld device built on Raspberry Pi 5 (7" TFT via HDMI) with a clean, responsive egui UI. Build and iterate effects fast by sorting pixels horizontally, vertically, or diagonally.

**Harpy** is a portable, kiosk-mode pixel sorting device designed for creative image manipulation through touch interaction.

## Why Rust?

- Fast native performance for real-time interaction
- Low memory footprint and predictable behavior
- Memory safety without garbage collection
- Portable development on desktop; deploy to Pi

## Features

### Core Functionality
- Live camera preview on Pi (rpicam-vid) with one-tap capture
- 3 algorithms: Horizontal, Vertical, Diagonal
- Edit phase with two controls:
  - Threshold slider (sensitivity of segment breaks)
  - Hue slider for optional tint (display-only)
- Crop phase with draggable handles; apply to turn crop into the new image
- Save & Iterate pipeline: auto-saves to `sorted_images/session_YYYYMMDD_HHMMSS/edit_XXX_*.png` and loads the last save as the new source
- USB export: copies entire `sorted_images/` to any mounted USB under `/media/*` or `/mnt/*`

### Touch-Optimized UI
- Large circular buttons (100-120px radius) for easy touch interaction
- Semi-transparent design with alpha blending for modern look
- Wide vertical sliders with oversized handles (0.8x width)
- Triple spacing between controls for fat-finger friendliness
- Hidden cursor in kiosk mode

### Production Features
- **Kiosk Mode**: Fullscreen operation at 1920x1080 with no window decorations
- **Sleep Mode**: Automatically dims to logo screen after 5 minutes of inactivity; touch anywhere to wake
- **Splash Screen**: 2-second fade-in logo on startup
- **Auto-Update**: Launcher script checks for updates from GitHub before starting
- **Auto-Start**: Systemd service for automatic launch on boot
- **Dual Exit Methods**: ESC key or 5 rapid taps in top-left corner (for touchscreen)

### Development
- Works on desktop for development (Windows/macOS/Linux) with animated test pattern when camera is unavailable
- Full UI functionality on any platform
- Cross-compilation support via `cross` tool

## About Harpy

**Harpy** is designed as a self-contained creative tool - a handheld pixel sorting device that artists can use anywhere. The kiosk mode and touch-optimized interface make it feel like a dedicated hardware device rather than a general-purpose computer.

### Development Philosophy

This is a **vibe coding project** - built through natural collaboration between human creativity and AI assistance. GitHub Copilot has been an integral part of the development process, helping to architect the touch-optimized UI, implement the kiosk mode features, and refine the user experience. The result is a tool that combines artistic vision with rapid iterative development, creating something that feels both polished and experimental.

## Quick Start

### Development (Desktop)

Requirements:
- Rust (stable)
- No additional dependencies

```powershell
# Windows/macOS/Linux
cargo run
```

Camera functionality is disabled on desktop; you'll see an animated test pattern instead.

### Deployment (Harpy Device / Raspberry Pi)

**Prerequisites:**
```bash
# Update system and install dependencies
sudo apt update && sudo apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install camera tools and GUI dependencies
sudo apt install -y rpicam-apps libgtk-3-dev libglib2.0-dev \
    libcairo2-dev libpango1.0-dev libgdk-pixbuf2.0-dev libatk1.0-dev
```

**Quick Install:**
```bash
# Clone and build on your Pi
git clone https://github.com/Vaghabund/Pixelsort.git
cd Pixelsort
cargo build --release

# Set up auto-start on boot (one-time setup)
cd deployment
chmod +x setup_autostart.sh
./setup_autostart.sh

# Enable auto-login for kiosk mode
sudo raspi-config
# Navigate to: System Options → Boot / Auto Login → Desktop Autologin
# Reboot when done
```

**Deployment Scripts:**

The `deployment/` folder contains 3 essential scripts:

1. **setup_autostart.sh** (27 lines) - One-time installer
   - Makes scripts executable
   - Installs systemd service for auto-start on boot
   - Run once during initial Pi setup
   
2. **start_pixelsort.sh** (60 lines) - Runtime launcher
   - Waits for X11 display server (adaptive polling, up to 30s)
   - Starts the Pixelsort app in fullscreen
   - Called automatically by systemd on boot
   
3. **update_and_rebuild.sh** (47 lines) - Background updater
   - Pulls latest code from GitHub
   - Rebuilds app (5-10 minutes)
   - Restarts service with error recovery
   - Triggered by "Pull & Restart" button in Developer menu

**Manual Control:**
```bash
# Start/stop service
sudo systemctl start pixelsort-kiosk.service
sudo systemctl stop pixelsort-kiosk.service

# Check status and logs
sudo systemctl status pixelsort-kiosk.service
journalctl -u pixelsort-kiosk.service -f
```

### Cross-Compilation (Recommended)

Build for Raspberry Pi from your development machine:

```powershell
# Install cross once
cargo install cross

# Ensure Docker is running, then build
cross build --release --target aarch64-unknown-linux-gnu
```

The binary will be at: `target/aarch64-unknown-linux-gnu/release/pixelsort-pi`

Transfer to your Pi and run the setup script to enable auto-start.

## UI Flow

- Input: Take Picture, Upload Image
- Edit: threshold + hue sliders; buttons for Algorithm, Sort Mode, Crop, Save & Iterate, New Image; optional Export to USB row when a drive is mounted
- Crop: drag corner handles; Apply Crop or Cancel

Notes
- Tint is applied as a display effect after sorting (doesn't change the source pixels until saved via Save & Iterate)
- Algorithm and Sort Mode cycle through predefined values

## Project Structure

The codebase is organized into domain-specific modules:

### Core Application
- **main.rs** - Application entry point, window setup, kiosk mode configuration, icon loading

### Hardware Layer (`src/hardware/`)
- **camera_controller.rs** - Raspberry Pi camera integration via rpicam-vid/rpicam-still
  - 30 FPS streaming with frame buffering
  - Snapshot capture with test pattern fallback for desktop
- **ups_monitor.rs** - Battery monitoring for UPS HAT (optional hardware)
  - I2C communication for battery status
  - Auto-shutdown on low battery

### Processing Layer (`src/processing/`)
- **pixel_sorter.rs** - Core sorting algorithms and pixel manipulation
  - Horizontal/Vertical/Diagonal sorting
  - Threshold-based segment detection
  - Hue-based tinting
- **image_ops.rs** - High-level image operations
  - Image loading and saving
  - Tint application and blending
  - Integration between sorting and image data
- **crop.rs** - Crop rectangle manipulation and application
  - Draggable crop handles
  - Apply crop with pixel sorting
- **texture.rs** - egui texture management for GPU rendering
  - Efficient texture updates for 30 FPS preview
  - Memory optimization for Pi hardware

### Session Management (`src/session/`)
- **manager.rs** - Save/load workflow and USB export
  - Auto-incrementing edit numbers (edit_001, edit_002, etc.)
  - Session directories by timestamp
  - USB drive detection and bulk export
  - Cross-platform directory operations

### System Control (`src/system/`)
- **update_manager.rs** - Git-based update checking and service restart
  - Checks for updates from GitHub origin/main
  - Spawns background rebuild script
  - Fallback mechanisms for error recovery
- **control.rs** - System-level controls
  - Application exit handling
  - 5-tap corner detection for touch exit

### User Interface (`src/ui/`)

The UI is structured for easy customization with quick-edit variables at the top of each file:

- **mod.rs** - Main UI coordinator and phase management
  - Three-phase workflow (Input → Edit → Crop)
  - Phase transitions and state management
  
- **state.rs** - UI state container
  - Current phase tracking
  - Image state (original, processed, cropped)
  - Slider values, algorithm selection
  
- **styles.rs** - Visual styling configuration (**edit colors and sizes here**)
  - Quick-edit constants for all colors (button alphas, slider RGB values)
  - Quick-edit constants for all sizes (button radii, spacing, slider width)
  - MenuStyle classes for popup menus
  - All visual appearance adjustable from top of file
  
- **components.rs** - UI component rendering (**edit animations and behavior here**)
  - Quick-edit constants for rendering (button animations, shadow offsets, fonts)
  - Quick-edit constants for effects (bubble opacity, label sizes)
  - Circular button rendering with hover/press states
  - Vertical slider rendering with touch-friendly handles
  - All component behavior adjustable from top of file
  
- **layouts.rs** - Spatial positioning (**edit positions and padding here**)
  - Quick-edit constants for layout (row offsets, padding multipliers)
  - Quick-edit constants for spacing (button positions, slider alignment)
  - Input phase layout (Take Picture, Upload buttons)
  - Edit phase layout (Algorithm/Mode/Crop/Iterate/New buttons + sliders)
  - Crop phase layout (Cancel/Apply buttons)
  - All positioning adjustable from top of file
  
- **screens.rs** - Phase-specific screen rendering
  - Input screen (camera preview, capture button)
  - Edit screen (image display, controls)
  - Crop screen (draggable handles)
  - Sleep screen (dim logo after 5 min idle)
  
- **menus.rs** - Popup menu system
  - Power menu (Exit/Restart)
  - Developer menu (Update/Restart)
  - USB export menu
  
- **helpers.rs** - UI utility functions
  - Layout helpers
  - Common UI patterns
  
- **indicators.rs** - Status indicators and overlays
  - Battery level display
  - Export status popups
  
- **viewport.rs** - Window and viewport management
  - Fullscreen configuration
  - Resolution handling (1920x1080)
  - Cursor hiding for kiosk mode
  
- **camera.rs** - Camera UI integration
  - Capture button logic
  - Phase transition after capture
  - Links UI to camera_controller

**Customization Made Easy:**
- Want to change **button colors**? → Edit constants at top of `styles.rs`
- Want to **move buttons**? → Edit constants at top of `layouts.rs`
- Want to change **button animations**? → Edit constants at top of `components.rs`

No need to hunt through code - all frequently adjusted values are grouped at the top of their respective files.

### Assets & Output
```
assets/
  Harpy_ICON.png        # App icon (splash screen + sleep mode)

sorted_images/          # Output directory (git-ignored)
  session_YYYYMMDD_HHMMSS/
    edit_001_horizontal.png
    edit_002_vertical.png
    ...
```

### Deployment Scripts
```
deployment/
  setup_autostart.sh         # One-time systemd service installer
  start_pixelsort.sh         # Runtime launcher (X11 wait + app start)
  update_and_rebuild.sh      # Background updater (git pull + rebuild)
  pixelsort-kiosk.service    # Systemd unit file
```

**Key Architecture Decisions:**
- **Modular design** - Each domain (hardware, processing, session, system, UI) is isolated
- **Immediate mode GUI** - egui renders UI from scratch each frame (no retained state)
- **Async camera** - tokio runtime handles camera streaming without blocking UI
- **Session-based workflow** - Each session is timestamped, edits auto-increment
- **Touch-first** - All controls designed for fat-finger interaction (100px+ targets)
- **Pi-optimized** - 30 FPS target, texture reuse, efficient memory management

## Kiosk Mode Details

When running on the Harpy device (Raspberry Pi), the app operates in kiosk mode to feel like a dedicated handheld:
- Fullscreen at 1920x1080 resolution
- No window decorations or title bar
- Cursor hidden for clean touch interaction
- Sleep mode activates after 5 minutes idle (shows dim Harpy logo)
- Touch anywhere to wake from sleep

**Exit Methods:**
- Press ESC key (if keyboard connected)
- Tap top-left corner 5 times rapidly (within 3 seconds)

## Troubleshooting

### Development
- **No camera on desktop**: App shows animated test pattern; capture button has no effect (this is normal)
- **Window size**: Runs at 1920x1080 on Pi; resizable on desktop for testing

### Raspberry Pi Deployment
- **Camera not working**: 
  ```bash
  # Install rpicam tools
  sudo apt install -y rpicam-apps
  
  # Test camera
  rpicam-hello
  vcgencmd get_camera
  ```

- **App won't start on boot**:
  ```bash
  # Check service status
  systemctl status pixelsort-kiosk.service
  
  # View logs
  journalctl -u pixelsort-kiosk.service -n 50
  
  # Test launcher manually
  cd ~/Pixelsort/deployment
  ./start_pixelsort.sh
  ```

- **Display resolution issues**:
  ```bash
  # Force resolution in /boot/config.txt
  sudo nano /boot/config.txt
  
  # Add these lines:
  hdmi_force_hotplug=1
  hdmi_group=2
  hdmi_mode=87
  hdmi_cvt=1920 1080 60 6 0 0 0
  ```

- **Build fails**:
  ```bash
  # Clean and rebuild
  cargo clean
  cargo build --release
  ```

- **Permission issues**:
  ```bash
  # Fix ownership
  sudo chown -R $USER:$USER ~/Pixelsort
  ```

- **USB export not detecting**:
  - USB drives must be mounted under `/media/*` or `/mnt/*`
  - Wait a few seconds after plugging in USB for auto-detection
  - Check mount status: `mount | grep media`

- **Can't exit app**: Use ESC key or tap top-left corner 5 times rapidly (within 3 seconds)

### Uninstall
```bash
# Stop and disable service
sudo systemctl stop pixelsort-kiosk.service
sudo systemctl disable pixelsort-kiosk.service
sudo rm /etc/systemd/system/pixelsort-kiosk.service
sudo systemctl daemon-reload

# Remove application
rm -rf ~/Pixelsort
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Version History

See [CHANGELOG.md](CHANGELOG.md) for release notes and version history.

## Contributing

Issues and PRs are welcome! Please ensure your code:
- Follows Rust idioms and formatting (`cargo fmt`)
- Passes all checks (`cargo clippy`)
- Is tested on desktop before submitting

## Links

- [UPS Setup Guide](docs/UPS_SETUP.md) - Hardware setup for battery monitoring (optional)
- [Battery Display Documentation](docs/BATTERY_DISPLAY.md) - Battery monitoring feature details
- [Copilot Instructions](.github/copilot-instructions.md) - Project architecture and development guidelines
- [GitHub Repository](https://github.com/Vaghabund/Pixelsort)