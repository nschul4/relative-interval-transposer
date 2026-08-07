@echo off
REM ==============================================================================
REM NOTE: Removing Directory Junctions inside 'C:\Program Files\' requires
REM Administrator privileges. Please run this script in an elevated Command Prompt
REM (Right-click -> "Run as administrator") or from an elevated Cygwin session.
REM ==============================================================================

:: Define the target directory environment variable
set "VST_DIR=C:\Program Files\Common Files\VST3\neal"

if exist "%VST_DIR%\midi-logger-vst.vst3" (
    rmdir "%VST_DIR%\midi-logger-vst.vst3"
)
if exist "%VST_DIR%\midi-transform-vst.vst3" (
    rmdir "%VST_DIR%\midi-transform-vst.vst3"
)

:: Optional: Remove the neal directory if it is empty
if exist "%VST_DIR%" (
    rmdir "%VST_DIR%" 2>nul
)