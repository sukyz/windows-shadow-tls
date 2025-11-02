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

# Show system information
Write-Host "System Information:" -ForegroundColor Cyan
Write-Host "OS: $([System.Environment]::OSVersion.VersionString)" -ForegroundColor White
Write-Host "Architecture: $($env:PROCESSOR_ARCHITECTURE)" -ForegroundColor White
Write-Host "Rust version: $(rustc --version)" -ForegroundColor White
Write-Host "Cargo version: $(cargo --version)" -ForegroundColor White
Write-Host ""

# Ensure we're targeting the correct Windows architecture
$target = "x86_64-pc-windows-msvc"
Write-Host "Build target: $target" -ForegroundColor Cyan

# Add the target if not already installed
Write-Host "Ensuring target is installed..." -ForegroundColor Yellow
rustup target add $target
Write-Host ""

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
cargo clean

# Build release version with explicit target
Write-Host "Building release version..." -ForegroundColor Yellow
cargo build --release --target $target --verbose

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
$binaryPath = "target\$target\release\shadow-tls-windows.exe"
if (Test-Path $binaryPath) {
    Write-Host "Binary location: $binaryPath" -ForegroundColor Cyan
    $fileInfo = Get-Item $binaryPath
    Write-Host "Binary size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Cyan
    
    # Copy to root directory for easy access
    $outputPath = "shadow-tls-windows-x86_64.exe"
    Copy-Item $binaryPath $outputPath -Force
    Write-Host "Copied binary to: $outputPath" -ForegroundColor Green
    
    Write-Host ""
    Write-Host "Testing binary..." -ForegroundColor Yellow
    try {
        $version = & $outputPath --version
        Write-Host "✅ Version test passed: $version" -ForegroundColor Green
        
        # Test help command
        $help = & $outputPath --help 2>&1
        if ($help -like "*Usage:*") {
            Write-Host "✅ Help test passed" -ForegroundColor Green
        } else {
            Write-Host "⚠️ Help test warning: unexpected output" -ForegroundColor Yellow
        }
        
        Write-Host ""
        Write-Host "🎉 Build and tests completed successfully!" -ForegroundColor Green
        Write-Host "📁 Windows binary ready: $outputPath" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Usage examples:" -ForegroundColor Yellow
        Write-Host "  Server: .\$outputPath server --listen 0.0.0.0:443 --server 127.0.0.1:8080 --tls cloudflare.com:443 --password your_password" -ForegroundColor White
        Write-Host "  Client: .\$outputPath client --listen 127.0.0.1:1080 --server your_server:443 --sni cloudflare.com --password your_password" -ForegroundColor White
        
    } catch {
        Write-Host "❌ Binary test failed: $($_.Exception.Message)" -ForegroundColor Red
        Write-Host "The binary may not be compatible with this system." -ForegroundColor Yellow
        
        # Run compatibility check if available
        if (Test-Path "check-windows-compat.ps1") {
            Write-Host "Running compatibility check..." -ForegroundColor Yellow
            & .\check-windows-compat.ps1 -BinaryPath $outputPath
        }
    }
} else {
    Write-Host "❌ Binary not found at expected location: $binaryPath" -ForegroundColor Red
    Write-Host "Please check the build output for errors." -ForegroundColor Yellow
}

Write-Host ""
Read-Host "Press Enter to exit"