#!/usr/bin/env bash

# Navigate to script root directory
cd "$(dirname "$0")" || exit 1

# Rebundle VST3 package in place
cargo nih-plug bundle midi-logger-vst || exit 1
cargo nih-plug bundle midi-transform-vst || exit 1

echo "[OK] Successfully updated VST3 bundle in target/bundled."