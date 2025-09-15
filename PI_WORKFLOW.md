# Raspberry Pi Development Workflow

## Your Development Setup

**Laptop (Windows)**: Development and coding  
**Raspberry Pi**: Building and running the graphics program  
**GitHub**: Version control and sync between devices  

## Daily Workflow

### On Your Laptop (Windows):

1. **Make changes** to your code in VS Code
2. **Deploy to Pi** using the PowerShell script:
   ```powershell
   # Deploy and build
   .\deploy_to_pi.ps1 -Message "Your commit message"
   
   # Deploy, build, and run
   .\deploy_to_pi.ps1 -Run -Message "Your commit message"
   ```

### On Your Raspberry Pi:

**Connect via SSH:**
```bash
ssh pixelsort@192.168.0.9
cd Pixelsort
```

**Quick commands:**
```bash
# Pull latest changes and run
./pi_dev.sh dev

# Just pull changes
./pi_dev.sh pull

# Just build
./pi_dev.sh build

# Just run (after building)
./pi_dev.sh run

# View logs
./pi_dev.sh logs
```

## Display Setup

- **Graphics appear on Pi's monitor** (not in SSH terminal)
- Make sure a monitor is connected to your Pi
- The program creates a window showing your pixel-sorted images
- You can control it via:
  - **MIDI controller** (if connected to Pi)
  - **Keyboard** (if keyboard connected to Pi)

## Debugging

If something doesn't work:

1. **Check SSH connection:**
   ```bash
   ssh pixelsort@192.168.0.9 "echo 'Connection OK'"
   ```

2. **Check if program runs:**
   ```bash
   ssh pixelsort@192.168.0.9 "cd Pixelsort && ./pi_dev.sh build"
   ```

3. **View build output for errors:**
   The build process shows any compilation errors

4. **Check Pi display:**
   Make sure DISPLAY=:0 is set and monitor is connected

## Project Structure

```
Laptop: c:\Users\Joel\personalpixelsort\
├── src/                    # Your Rust source code
├── deploy_to_pi.ps1       # Deployment script
└── pi_dev.sh              # Pi development helper

Pi: /home/pixelsort/Pixelsort/
├── src/                    # Synced from GitHub
├── target/release/         # Compiled ARM binary
└── pi_dev.sh              # Development helper script
```

## Controls

**Keyboard Controls:**
- Up/Down arrows: Adjust threshold
- M: Switch sort mode (Brightness/Black/White)
- N: Switch direction (horizontal/vertical) 
- B: Toggle random mode
- Enter: Save current image

**Future Hardware Controls (planned):**
- Physical potentiometers for threshold adjustment
- Physical buttons for mode switching and control

## Tips

- Always **commit and push** changes before deploying
- Use meaningful commit messages
- Test locally on Windows first when possible
- Keep your Pi connected to power and network
- The first build takes longest (~3 minutes), subsequent builds are faster
