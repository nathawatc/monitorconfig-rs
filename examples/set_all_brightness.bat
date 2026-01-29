@echo off
REM Example: Set brightness across all monitors

set BRIGHTNESS=%1
if "%BRIGHTNESS%"=="" set BRIGHTNESS=75

echo Setting all monitors to brightness: %BRIGHTNESS%
echo.

REM Get list of monitors and set brightness
for /f "tokens=*" %%i in ('monitor-config list --json') do (
    echo %%i | findstr "DISPLAY" >nul
    if not errorlevel 1 (
        for /f "tokens=2 delims=:" %%j in ("%%i") do (
            set DEVICE=%%j
            set DEVICE=!DEVICE:"=!
            set DEVICE=!DEVICE: =!
            set DEVICE=!DEVICE:,=!
            echo Setting !DEVICE! to %BRIGHTNESS%...
            monitor-config set-brightness %BRIGHTNESS% --device "!DEVICE!"
        )
    )
)

echo.
echo Done! All monitors set to brightness %BRIGHTNESS%
