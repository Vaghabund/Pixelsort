#!/usr/bin/env python3
"""
Simple battery voltage reader for UPS HAT
Reads from I2C device at 0x42 and outputs voltage
"""
import sys
import struct

try:
    # Try to import smbus for I2C communication
    import smbus
    
    # I2C bus 1, address 0x42
    bus = smbus.SMBus(1)
    address = 0x42
    
    # Read voltage register (0x02) as word (2 bytes)
    # This is specific to INA219-based UPS HATs
    data = bus.read_word_data(address, 0x02)
    
    # Swap bytes (little-endian to big-endian)
    data = ((data & 0xFF) << 8) | ((data & 0xFF00) >> 8)
    
    # Convert to voltage (INA219: LSB = 4mV)
    voltage = (data >> 3) * 0.004
    
    # Check if charging (read from another register or GPIO)
    # For now, assume not charging if voltage < 8.2V (not fully charged)
    is_charging = voltage < 8.2
    
    # Output: voltage,charging (e.g., "7.85,0")
    print(f"{voltage:.2f},{1 if is_charging else 0}")
    sys.exit(0)
    
except ImportError:
    # smbus not available
    print("0.0,0", file=sys.stderr)
    sys.exit(1)
except Exception as e:
    # Any other error
    print(f"0.0,0", file=sys.stderr)
    sys.exit(1)
