@echo off
:: Remove from user environment variables registry
REG delete "HKCU\Environment" /v NIH_LOG /f > nul 2>&1

:: Clear from current session
set "NIH_LOG="

echo [OK] NIH_LOG has been unset.
echo [*] Note: Restart Ableton Live for the change to take effect.
