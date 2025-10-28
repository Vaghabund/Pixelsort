# UPS Battery Monitor Setup

Harpy includes safe shutdown support for I2C-based UPS HATs to prevent data corruption when battery runs low.

## Features

- Monitors battery voltage in real-time
- Automatic safe shutdown at configured voltage threshold
- 30-second warning before shutdown
- Graceful application exit
- System logs for debugging

## Configuration

Edit `ups_config.toml` in the project root:

```toml
[ups]
# Enable/disable UPS monitoring
enabled = true

# I2C bus (usually 1 on Raspberry Pi)
i2c_bus = 1

# I2C address of battery fuel gauge
# Common addresses: 0x36 (MAX17048), 0x55 (BQ27441)
# Use `i2cdetect -y 1` to find your device
i2c_address = 0x36

# Shutdown voltage threshold (volts)
# For 2S Li-ion (7.4V nominal): 6.4V safe minimum
# Adjust based on your battery configuration
voltage_threshold = 6.4

# Check interval (seconds)
check_interval_secs = 10
```

## Finding Your UPS I2C Address

On your Raspberry Pi, install i2c-tools and scan for devices:

```bash
sudo apt install -y i2c-tools
sudo i2cdetect -y 1
```

Look for devices at common addresses:
- `0x36` - MAX17048/MAX17049 fuel gauge
- `0x55` - BQ27441 fuel gauge  
- `0x5A` - DFRobot Solar Power Manager

## Testing

### Check if UPS is detected:

```bash
# View system power supply info
cat /sys/class/power_supply/*/uevent

# Check if on battery
cat /sys/class/power_supply/AC/online  # 0 = battery, 1 = AC
```

### Monitor application logs:

```bash
# Watch logs in real-time
journalctl -u pixelsort-kiosk.service -f

# Look for UPS monitoring messages
journalctl -u pixelsort-kiosk.service | grep -i ups
journalctl -u pixelsort-kiosk.service | grep -i battery
```

## How It Works

1. **Background Monitoring**: UPS monitor runs as async task, checking battery every 10 seconds
2. **Low Battery Detection**: When voltage drops below threshold, 30-second countdown starts
3. **Safe Shutdown Sequence**:
   - Application saves any work in progress
   - Camera stream closes gracefully
   - Application exits cleanly
   - System shutdown initiated: `sudo shutdown -h now`

## Voltage Guidelines

### 2S Li-ion (7.4V nominal):
- **Full**: 8.4V
- **Normal**: 7.4V
- **Low**: 6.8V
- **Critical**: 6.4V (recommended threshold)
- **Cutoff**: 6.0V (do not discharge below)

### Single-cell Li-ion (3.7V nominal):
- **Full**: 4.2V
- **Normal**: 3.7V
- **Low**: 3.4V
- **Critical**: 3.2V (recommended threshold)
- **Cutoff**: 3.0V (do not discharge below)

## Disabling UPS Monitoring

Set `enabled = false` in `ups_config.toml`, or delete the file to use defaults (disabled).

## Troubleshooting

### "Battery voltage unavailable"
- UPS might not expose voltage via sysfs
- I2C address might be wrong - use `i2cdetect`
- I2C not enabled - run `sudo raspi-config` → Interface Options → I2C → Enable

### Premature shutdowns
- Voltage threshold too high
- Battery degraded/aging
- Increase `voltage_threshold` in config

### No shutdown when battery low
- `enabled = false` in config
- Incorrect I2C address
- UPS not properly connected
- Check logs: `journalctl -u pixelsort-kiosk.service -n 100`

### Permission issues
The systemd service runs as user `pixelsort` with sudo permissions for shutdown command.

## Manual Testing

Temporarily lower threshold to test (don't drain battery):

```toml
# Test configuration (will trigger on normal voltage)
voltage_threshold = 99.0  # Unrealistically high - will trigger immediately
check_interval_secs = 5
```

Watch logs while app runs:
```bash
journalctl -u pixelsort-kiosk.service -f
```

You should see warnings and countdown before shutdown.

**Remember to restore normal threshold after testing!**
