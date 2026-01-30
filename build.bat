@echo off
REM Build script for monitorconfig on Windows

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
echo.

REM Check if GUI subsystem build is requested
if "%1"=="--gui" (
    echo Building with GUI subsystem ^(no console window^)...
    cargo build --release --features gui-subsystem
) else if "%1"=="gui" (
    echo Building with GUI subsystem ^(no console window^)...
    cargo build --release --features gui-subsystem
) else (
    echo Building standard version...
    cargo build --release
)

if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    exit /b 1
)

echo.
echo Build complete!
echo.
echo Binary location: target\release\monitorconfig.exe
echo.

REM Show binary size
for %%A in (target\release\monitorconfig.exe) do echo Binary size: %%~zA bytes
echo.

if "%1"=="--gui" (
    echo Note: This build uses GUI subsystem and won't show console windows.
    echo.
) else if "%1"=="gui" (
    echo Note: This build uses GUI subsystem and won't show console windows.
    echo.
)

echo To install globally, run: cargo install --path .
echo For GUI subsystem build, run: build.bat --gui
echo.

REM Run basic check
echo Running basic functionality check...
echo.
target\release\monitorconfig.exe --version
echo.
echo To test the tool, run: target\release\monitorconfig.exe list
