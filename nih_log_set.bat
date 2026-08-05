@echo off
set "LOG_PATH=%~dp0tmp_nih_log.txt"

:: Set system-wide environment variable
setx NIH_LOG "%LOG_PATH%" > nul

if %ERRORLEVEL% EQU 0 (
    :: Set for the current running session immediately
    set "NIH_LOG=%LOG_PATH%"
    echo [OK] NIH_LOG successfully set to: %LOG_PATH%
    echo [*] Note: Restart Ableton Live for the change to take effect.
) else (
    echo [ERROR] Failed to set NIH_LOG environment variable.
)
