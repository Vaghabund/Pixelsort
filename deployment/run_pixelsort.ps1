# Harpy Pixel Sorter - Auto-update launcher script (Windows version)
# This script updates the app from git and runs it

$APP_DIR = "C:\Users\joel\Pixelsort"
# Repository: https://github.com/Vaghabund/Pixelsort

Set-Location $APP_DIR

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Harpy Pixel Sorter - Starting..." -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Save current HEAD for comparison
$LOCAL_BEFORE = git rev-parse HEAD 2>$null
if (-not $LOCAL_BEFORE) { $LOCAL_BEFORE = "unknown" }

$NEEDS_REBUILD = $false

# Check for internet connectivity
try {
    $null = Test-Connection -ComputerName github.com -Count 1 -TimeoutSeconds 2 -ErrorAction Stop
    Write-Host "Internet connected. Checking for updates..." -ForegroundColor Green
    
    # Fetch latest changes
    git fetch origin main 2>$null
    
    # Check if we're behind
    $LOCAL = git rev-parse HEAD 2>$null
    $REMOTE = git rev-parse origin/main 2>$null
    
    if (($LOCAL -ne $REMOTE) -and ($REMOTE -ne $null)) {
        Write-Host "Updates found! Updating..." -ForegroundColor Yellow
        
        # Stash any local changes
        git stash --include-untracked 2>$null | Out-Null
        
        # Pull changes
        $pullResult = git pull origin main 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✓ Update successful!" -ForegroundColor Green
            
            # Get new HEAD after pull
            $LOCAL_AFTER = git rev-parse HEAD 2>$null
            
            # Check if source code changed between before and after
            $changedFiles = git diff --name-only $LOCAL_BEFORE $LOCAL_AFTER
            if ($changedFiles -match "^src/|Cargo.toml") {
                Write-Host "Source code changed. Will rebuild..." -ForegroundColor Yellow
                $NEEDS_REBUILD = $true
            } else {
                Write-Host "No source changes. Using existing binary." -ForegroundColor Green
            }
        } else {
            Write-Host "⚠ Update failed. Using current version." -ForegroundColor Yellow
        }
    } else {
        Write-Host "Already up to date." -ForegroundColor Green
    }
} catch {
    Write-Host "No internet connection. Skipping update check." -ForegroundColor Yellow
}

Write-Host "Starting Pixel Sorter..." -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Run the application
if ($NEEDS_REBUILD) {
    Write-Host "Building updated code..." -ForegroundColor Yellow
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Build successful!" -ForegroundColor Green
    } else {
        Write-Host "✗ Build failed!" -ForegroundColor Red
    }
}

# Run (will use existing binary if build failed)
cargo run --release

# If app exits, wait a moment before this script ends
Write-Host ""
Write-Host "Application closed." -ForegroundColor Cyan
Start-Sleep -Seconds 2
