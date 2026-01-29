@echo off
REM Build script for monitor-config on Windows

echo Building MonitorConfig Rust CLI...
echo ==================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust is not installed. Please install from https://rustup.rs/
    exit /b 1
)

REM Show Rust version
echo Rust version:
rustc --version
cargo --version
echo.

REM Build in release mode
echo Building in release mode...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    exit /b 1
)

echo.
echo Build complete!
echo.
echo Binary location: target\release\monitor-config.exe
echo.

REM Show binary size
for %%A in (target\release\monitor-config.exe) do echo Binary size: %%~zA bytes
echo.
echo To install globally, run: cargo install --path .
echo.

REM Run basic check
echo Running basic functionality check...
echo.
target\release\monitor-config.exe --version
echo.
echo To test the tool, run: target\release\monitor-config.exe list
