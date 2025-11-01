#!/bin/bash
# One-time setup script for Pixelsort auto-start on boot

echo "Installing Pixelsort auto-start service..."

# Check if we're in the right directory
if [ ! -f "start_pixelsort.sh" ]; then
    echo "Error: start_pixelsort.sh not found. Run from deployment directory."
    exit 1
fi

# Make scripts executable
chmod +x start_pixelsort.sh update_and_rebuild.sh

# Install systemd service
sudo cp pixelsort-kiosk.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable pixelsort-kiosk.service

echo "✓ Setup complete! Service will start on boot."
echo ""
echo "Useful commands:"
echo "  Start now:   sudo systemctl start pixelsort-kiosk.service"
echo "  Stop:        sudo systemctl stop pixelsort-kiosk.service"
echo "  Status:      sudo systemctl status pixelsort-kiosk.service"
echo "  View logs:   journalctl -u pixelsort-kiosk.service -f"
echo ""
echo "Next steps: See README.md for auto-login setup (raspi-config)"
