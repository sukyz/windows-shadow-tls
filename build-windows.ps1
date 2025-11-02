#!/usr/bin/env pwsh
# PowerShell build script for Shadow TLS Windows

Write-Host "Building Shadow TLS for Windows..." -ForegroundColor Green
Write-Host ""

# Check if Rust is installed
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Rust/Cargo not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "Rust version:" -ForegroundColor Yellow
cargo --version
Write-Host ""

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
cargo clean

# Build release version
Write-Host "Building release version..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "Build failed! Please check the error messages above." -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "Build successful!" -ForegroundColor Green
Write-Host ""

# Show binary info
$binaryPath = "target\release\shadow-tls-windows.exe"
if (Test-Path $binaryPath) {
    Write-Host "Binary location: $binaryPath" -ForegroundColor Cyan
    $fileInfo = Get-Item $binaryPath
    Write-Host "Binary size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Cyan
    
    Write-Host ""
    Write-Host "Testing binary..." -ForegroundColor Yellow
    & $binaryPath --version
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "Binary test successful!" -ForegroundColor Green
    } else {
        Write-Host ""
        Write-Host "Warning: Binary test failed!" -ForegroundColor Yellow
    }
} else {
    Write-Host "Warning: Binary not found at expected location!" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Build complete! You can now use $binaryPath" -ForegroundColor Green
Read-Host "Press Enter to exit"