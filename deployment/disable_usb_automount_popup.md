# Disable USB Automount Popup on Raspberry Pi

To prevent the file manager from opening automatically when a USB drive is inserted, you need to disable the automount feature in the file manager settings.

## For Raspberry Pi OS with Desktop

### Option 1: Via File Manager Settings (PCManFM)
1. Open the file manager
2. Go to `Edit` → `Preferences`
3. Go to the `Volume Management` tab
4. **Uncheck** "Show available options for removable media when they are inserted"
5. **Uncheck** "Mount removable media automatically when they are inserted"
6. Click OK

### Option 2: Via dconf-editor (if available)
```bash
sudo apt-get install dconf-editor
dconf-editor
```
Navigate to: `/org/gnome/desktop/media-handling/`
- Set `automount` to `false`
- Set `automount-open` to `false`

### Option 3: Via Command Line (Persistent)
```bash
# Create config directory if it doesn't exist
mkdir -p ~/.config/pcmanfm/LXDE-pi

# Edit or create the config file
nano ~/.config/pcmanfm/LXDE-pi/pcmanfm.conf
```

Add or modify the `[volume]` section:
```ini
[volume]
mount_on_startup=0
mount_removable=0
autorun=0
```

Save and exit (Ctrl+X, Y, Enter)

### Option 4: Disable for the kiosk user
Since the app runs as the `pixelsort` user in kiosk mode, disable it for that user:

```bash
sudo -u pixelsort mkdir -p /home/pixelsort/.config/pcmanfm/LXDE-pi
sudo nano /home/pixelsort/.config/pcmanfm/LXDE-pi/pcmanfm.conf
```

Add:
```ini
[volume]
mount_on_startup=0
mount_removable=0
autorun=0
```

## Verification
1. Restart the Pi or logout/login
2. Insert a USB drive
3. The file manager should NOT open automatically
4. The Harpy app will still detect the USB and show the internal export dialog

## Troubleshooting
If the popup still appears:
- Check if another file manager is running (Thunar, Nautilus, etc.)
- Make sure you edited the config for the correct user (`pixelsort`)
- Try rebooting the Pi after making changes
