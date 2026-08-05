@echo off
REM ==============================================================================
REM NOTE: Creating Directory Junctions inside 'C:\Program Files\' requires 
REM Administrator privileges. Please run this script in an elevated Command Prompt 
REM (Right-click -> "Run as administrator") or from an elevated Cygwin session.
REM ==============================================================================

mklink /J "C:\Program Files\Common Files\VST3\neal\midi-logger-vst.vst3" "%~dp0target\bundled\midi-logger-vst.vst3"
mklink /J "C:\Program Files\Common Files\VST3\neal\midi-transform-vst.vst3" "%~dp0target\bundled\midi-transform-vst.vst3"
