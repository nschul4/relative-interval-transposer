# Relative Interval Transposer

A VST3 MIDI plugin workspace built in Rust using the [NIH-plug](https://github.com/robbert-vdh/nih-plug) framework.

The primary plugin intercepts incoming MIDI notes and dynamically transposes pitch based on an interval played against a held **Root Note**. Striking offset keys while holding the root calculates the interval and steps the transposition outward on each subsequent trigger.

---

## Workspace Layout

```text
.
├── Cargo.toml                  # Workspace manifest
├── mklink.bat                  # Windows VST3 directory junction helper
├── checklink.bat               # VST3 junction verification helper
├── safe-build.sh               # In-place VST3 bundle update script
└── subprojects/
    ├── cli-sanity/             # Toolchain verification CLI
    ├── midi-logger-vst/        # Diagnostic MIDI pass-through VST3
    └── midi-transform-vst/     # Relative Interval Transposer plugin

```

---

## Quickstart

### Prerequisites

* **Rust:** `x86_64-pc-windows-msvc` toolchain
* **Bundler:** `cargo install cargo-nih-plug`
* **DAW Host:** VST3-compatible host (tested in Ableton Live)

### Build & Link

1. **Build VST3 Bundles:**
```bash
cargo nih-plug bundle midi-transform-vst
cargo nih-plug bundle midi-logger-vst

```


2. **Link to System VST3 Directory (Windows):**
Run `mklink.bat` from an elevated Command Prompt or Cygwin session to create junctions pointing to `target/bundled/`.
3. **In-Place Rebuilds without DAW Restart:**
Run `./safe-build.sh` to update binary artifacts in place.

---

## How It Works

1. **Root Assignment:** Play a single note (e.g., **C3**). The note passes through normally and sets `Root = C3`.
2. **Interval Step:** While holding **C3**, play an offset note (e.g., **E3**, +4 semitones). Output transposes to **E3**.
3. **Accumulate:** Re-strike **E3** while holding **C3**. Pitch advances another +4 semitones to **G#3**.
4. **Reset:** Release all keys to reset state.

---

## Telemetry & Logging

* **Build Metadata:** Builds automatically embed short Git commit hashes and UTC timestamps via `build.rs`.
* **Runtime Logs:** Run **DebugView** (`Dbgview.exe`) with *Capture Win32* enabled to view real-time state changes and event logs.
