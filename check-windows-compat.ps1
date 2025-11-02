# Windows Compatibility Check Script
# This script checks if the Shadow TLS binary is compatible with Windows

param(
    [string]$BinaryPath = ".\shadow-tls-windows-x86_64.exe"
)

Write-Host "=== Shadow TLS Windows Compatibility Check ===" -ForegroundColor Green
Write-Host ""

# Check if file exists
if (-not (Test-Path $BinaryPath)) {
    Write-Host "❌ Binary file not found: $BinaryPath" -ForegroundColor Red
    Write-Host "Please download the binary from: https://github.com/sukyz/windows-shadow-tls/releases" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Binary file found: $BinaryPath" -ForegroundColor Green

# Check file properties
$fileInfo = Get-Item $BinaryPath
Write-Host "📁 File size: $($fileInfo.Length) bytes" -ForegroundColor Cyan
Write-Host "📅 File date: $($fileInfo.LastWriteTime)" -ForegroundColor Cyan

# Check if it's a valid PE executable
try {
    $fileHeader = Get-Content $BinaryPath -Encoding Byte -TotalCount 2
    if ($fileHeader[0] -eq 0x4D -and $fileHeader[1] -eq 0x5A) {
        Write-Host "✅ Valid PE executable (MZ header found)" -ForegroundColor Green
    } else {
        Write-Host "❌ Invalid executable format" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "❌ Error reading file header: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Check Windows version compatibility
$osVersion = [System.Environment]::OSVersion.Version
Write-Host "🖥️  Windows version: $($osVersion.Major).$($osVersion.Minor) (Build $($osVersion.Build))" -ForegroundColor Cyan

if ($osVersion.Major -lt 10) {
    Write-Host "⚠️  Warning: Windows 10 or later is recommended" -ForegroundColor Yellow
} else {
    Write-Host "✅ Windows version is compatible" -ForegroundColor Green
}

# Check architecture
$arch = $env:PROCESSOR_ARCHITECTURE
Write-Host "🏗️  System architecture: $arch" -ForegroundColor Cyan

if ($arch -ne "AMD64") {
    Write-Host "⚠️  Warning: This binary is built for x64 systems" -ForegroundColor Yellow
} else {
    Write-Host "✅ Architecture is compatible" -ForegroundColor Green
}

# Try to run the binary
Write-Host ""
Write-Host "🧪 Testing binary execution..." -ForegroundColor Yellow

try {
    $output = & $BinaryPath --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Binary executed successfully" -ForegroundColor Green
        Write-Host "📋 Version output: $output" -ForegroundColor Cyan
    } else {
        Write-Host "❌ Binary execution failed with exit code: $LASTEXITCODE" -ForegroundColor Red
        Write-Host "📋 Error output: $output" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Failed to execute binary: $($_.Exception.Message)" -ForegroundColor Red
    
    # Check for common issues
    if ($_.Exception.Message -like "*not compatible*") {
        Write-Host ""
        Write-Host "🔧 Troubleshooting suggestions:" -ForegroundColor Yellow
        Write-Host "1. Make sure you downloaded the correct x64 version" -ForegroundColor White
        Write-Host "2. Try running as Administrator" -ForegroundColor White
        Write-Host "3. Check Windows Defender or antivirus software" -ForegroundColor White
        Write-Host "4. Install Visual C++ Redistributable if missing" -ForegroundColor White
    }
}

# Check for required DLLs
Write-Host ""
Write-Host "🔍 Checking for required system libraries..." -ForegroundColor Yellow

$requiredDlls = @(
    "kernel32.dll",
    "user32.dll", 
    "ws2_32.dll",
    "advapi32.dll",
    "bcrypt.dll"
)

foreach ($dll in $requiredDlls) {
    $dllPath = Join-Path $env:SystemRoot "System32\$dll"
    if (Test-Path $dllPath) {
        Write-Host "✅ $dll found" -ForegroundColor Green
    } else {
        Write-Host "❌ $dll missing" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "=== Compatibility Check Complete ===" -ForegroundColor Green
Write-Host ""
Write-Host "If you encounter issues, please:" -ForegroundColor Yellow
Write-Host "1. Check the GitHub Issues: https://github.com/sukyz/windows-shadow-tls/issues" -ForegroundColor White
Write-Host "2. Review the documentation: https://github.com/sukyz/windows-shadow-tls/blob/main/QUICK-START.md" -ForegroundColor White
Write-Host "3. Try running with administrator privileges" -ForegroundColor White