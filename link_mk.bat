@echo off
REM ==============================================================================
REM NOTE: Creating Directory Junctions inside 'C:\Program Files\' requires
REM Administrator privileges. Please run this script in an elevated Command Prompt
REM (Right-click -> "Run as administrator") or from an elevated Cygwin session.
REM ==============================================================================

:: Define the target directory environment variable
set "VST_DIR=C:\Program Files\Common Files\VST3\Neal"

if not exist "%VST_DIR%" mkdir "%VST_DIR%"
mklink /J "%VST_DIR%\midi-logger-vst.vst3" "%~dp0target\bundled\midi-logger-vst.vst3"
mklink /J "%VST_DIR%\midi-transform-vst.vst3" "%~dp0target\bundled\midi-transform-vst.vst3"