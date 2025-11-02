@echo off
REM Windows build script for Shadow TLS Windows

echo Building Shadow TLS for Windows...
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Rust/Cargo not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo Rust version:
cargo --version
echo.

REM Clean previous builds
echo Cleaning previous builds...
cargo clean

REM Build release version
echo Building release version...
cargo build --release

if %ERRORLEVEL% neq 0 (
    echo.
    echo Build failed! Please check the error messages above.
    pause
    exit /b 1
)

echo.
echo Build successful!
echo.
echo Binary location: target\release\shadow-tls-windows.exe
echo Binary size:
dir target\release\shadow-tls-windows.exe | findstr shadow-tls-windows.exe

echo.
echo Testing binary...
target\release\shadow-tls-windows.exe --version

if %ERRORLEVEL% neq 0 (
    echo.
    echo Warning: Binary test failed!
) else (
    echo.
    echo Binary test successful!
)

echo.
echo Build complete! You can now use target\release\shadow-tls-windows.exe
pause