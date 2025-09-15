# PowerShell script to deploy to Raspberry Pi
# Usage: .\deploy_to_pi.ps1

param(
    [switch]$Build,
    [switch]$Run,
    [string]$Message = "Update from laptop"
)

Write-Host "Deploying to Raspberry Pi..." -ForegroundColor Green

# 1. Push changes to GitHub
Write-Host "Pushing to GitHub..." -ForegroundColor Yellow
git add .
git commit -m $Message
git push origin main

if ($LASTEXITCODE -ne 0) {
    Write-Host "Git push failed!" -ForegroundColor Red
    exit 1
}

# 2. Pull and build on Pi
Write-Host "Pulling and building on Pi..." -ForegroundColor Yellow
$sshCommand = @"
cd Pixelsort && 
git pull origin main && 
source ~/.cargo/env && 
cargo build --release
"@

ssh pixelsort@192.168.0.9 $sshCommand

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build on Pi failed!" -ForegroundColor Red
    exit 1
}

Write-Host "Deployment successful!" -ForegroundColor Green

# 3. Run if requested
if ($Run) {
    Write-Host "Running program on Pi..." -ForegroundColor Cyan
    Write-Host "Note: Graphics will appear on Pi's display, not here!" -ForegroundColor Yellow
    ssh pixelsort@192.168.0.9 "cd Pixelsort && source ~/.cargo/env && DISPLAY=:0 ./target/release/pixelsort"
}
