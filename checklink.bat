@echo off
REM ==============================================================================
REM NOTE: Checking Directory Junctions inside 'C:\Program Files\' does not 
REM strictly require Administrator privileges, but running this script in an 
REM elevated Command Prompt (Right-click -> "Run as administrator") or from an 
REM elevated Cygwin session ensures consistent access.
REM ==============================================================================

if exist "C:\Program Files\Common Files\VST3\neal\midi-logger-vst.vst3" (
    echo [FOUND] C:\Program Files\Common Files\VST3\neal\midi-logger-vst.vst3
) else (
    echo [MISSING] C:\Program Files\Common Files\VST3\neal\midi-logger-vst.vst3
)

if exist "C:\Program Files\Common Files\VST3\neal\midi-transform-vst.vst3" (
    echo [FOUND] C:\Program Files\Common Files\VST3\neal\midi-transform-vst.vst3
) else (
    echo [MISSING] C:\Program Files\Common Files\VST3\neal\midi-transform-vst.vst3
)