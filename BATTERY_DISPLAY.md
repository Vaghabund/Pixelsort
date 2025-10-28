# UPS Battery Display - Implementation Summary

## What Was Added

### 1. Battery Status Monitoring
- Real-time voltage reading from UPS via sysfs
- Charging status detection
- Battery percentage calculation
- Global battery state accessible throughout the app

### 2. UI Battery Indicator
Located in **top-right corner** of screen with:
- **Battery icon** - Visual representation with fill level
- **Percentage** - Current battery level (0-100%)
- **Voltage** - Actual voltage reading (e.g., 7.8V)
- **Charging indicator** - ⚡ symbol when plugged in
- **Color coding**:
  - 🟢 Green = Charging
  - 🔴 Red = Below 20% (critical)
  - 🟡 Yellow = 20-40% (low)
  - ⚫ Grey = Above 40% (normal)

### 3. Safe Shutdown Integration
- Monitors battery continuously
- Triggers 30-second warning when voltage drops below threshold
- Gracefully closes app and shuts down system
- Prevents shutdown while charging

## Files Modified

1. **src/ups_monitor.rs** - Enhanced with:
   - `BatteryStatus` struct with voltage, percentage, charging state
   - `get_battery_status()` - Public API to read battery info
   - Charging detection from `/sys/class/power_supply/*/status`
   - Voltage-to-percentage conversion
   - Global state using `lazy_static`

2. **src/ui.rs** - Added:
   - `render_battery_indicator()` - Draws battery widget
   - Positioned at top-right with padding
   - Semi-transparent background
   - Updates in real-time

3. **src/main.rs** - Added:
   - UPS monitoring initialization on startup
   - Config file loading

4. **Cargo.toml** - Added:
   - `lazy_static = "1.4"` for global state

5. **ups_config.toml** - User config (gitignored)

6. **ups_config.toml.template** - Template with documentation

## Usage

### On Raspberry Pi:
The battery indicator will automatically appear when:
1. UPS monitoring is enabled in `ups_config.toml`
2. Battery voltage is readable from sysfs
3. The app detects UPS hardware

### On Development Machine:
- Shows simulated battery at ~75% (7.8V, not charging)
- Useful for testing UI layout

## Configuration

Edit `ups_config.toml`:

```toml
[ups]
enabled = true                # Set to true to show battery
i2c_bus = 1
i2c_address = 0x36
voltage_threshold = 6.4       # Shutdown voltage
check_interval_secs = 10      # Update frequency
```

## Testing

### Check if working:
```bash
# View battery info
cat /sys/class/power_supply/battery/voltage_now
cat /sys/class/power_supply/battery/status

# Run app and look for UPS logs
journalctl -u pixelsort-kiosk.service -f | grep -i battery
```

### Expected behavior:
- Battery icon appears in top-right corner
- Updates every 10 seconds
- Shows current voltage and percentage
- Changes color based on charge level
- Shows ⚡ when charging

## Next Steps

To deploy to your Pi:
1. Copy `ups_config.toml.template` to `ups_config.toml`
2. Edit config with your UPS settings
3. Find I2C address: `sudo i2cdetect -y 1`
4. Enable monitoring: `enabled = true`
5. Build and run: `cargo build --release`

The battery display will now show in your Harpy device!
