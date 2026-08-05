#!/usr/bin/env bash

# Navigate to script root directory
cd "$(dirname "$0")" || exit 1

# 1. Compile the VST3 library target
cargo build -p midi-transform-vst || exit 1

# 2. Ensure target VST3 bundle directory structure exists
BUNDLE_DIR="target/bundled/midi-transform-vst.vst3/Contents/x86_64-win"
mkdir -p "$BUNDLE_DIR"

# 3. Copy compiled binary into bundle
cp target/debug/midi_transform_vst.dll "$BUNDLE_DIR/midi-transform-vst.vst3" 2>/dev/null || \
cp target/debug/libmidi_transform_vst.so "$BUNDLE_DIR/midi-transform-vst.vst3" 2>/dev/null || \
cp target/debug/midi_transform_vst.cdylib "$BUNDLE_DIR/midi-transform-vst.vst3" 2>/dev/null

echo "[OK] Successfully updated VST3 bundle without restarting DAW."
