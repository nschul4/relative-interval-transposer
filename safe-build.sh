#!/usr/bin/env bash

# Navigate to script root directory
cd "$(dirname "$0")" || exit 1

# 1. Compile the VST3 library target
cargo build -p midi-transform-vst || exit 1

# 2. Paths to compiled DLL and destination bundle target
DLL_SRC="target/debug/midi_transform_vst.dll"
BUNDLE_TARGET="target/bundled/midi-transform-vst.vst3/Contents/x86_64-win/midi-transform-vst.vst3"

# 3. Rename locked binary if present (bypasses Windows DLL lock), then copy new DLL
if [ -f "$BUNDLE_TARGET" ]; then
    mv -f "$BUNDLE_TARGET" "${BUNDLE_TARGET}.old" 2>/dev/null
fi

cp -f "$DLL_SRC" "$BUNDLE_TARGET"
echo "[OK] Successfully updated VST3 bundle without restarting DAW."
