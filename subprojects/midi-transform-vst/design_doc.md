# System Design Document: Relative Interval Transposer (MIDI FX)

## 1. System Architecture & Toolchain

### 1.1 Overview

The project is a cross-platform (Windows-first) MIDI-in / MIDI-out VST3 audio plugin workspace developed in **Rust** using the **NIH-plug** framework.

### 1.2 Development Environment Configuration

* **Primary OS:** Windows 10/11
* **Terminal Environment:** Cygwin (Bash) driving native Windows/Rust toolchains (`x86_64-pc-windows-msvc`)
* **IDE / Editor:** Visual Studio Code
* **Target Host / DAW:** Ableton Live (Windows VST3 host)
* **Debugging & Telemetry:**
  * **Realtime Plugin State & Logic:** Logged via `nih_log!` and monitored using Microsoft DebugView (`Dbgview.exe` / `dbgviewcli64.exe`) or written directly to file via `NIH_LOG`.
  * **Host & VST3 Instantiation Troubleshooting:** Inspect Ableton's native application diagnostic log at:
    `C:\Users\austyn\AppData\Roaming\Ableton\Live 11.0.11\Preferences\Log.txt`
    *(Crucial for diagnosing issue scenarios where the VST3 fails to scan, crashes on instantiation, or silently rejects being dragged onto a track).*
* **Realtime Safety:** The processing thread avoids dynamic heap allocations (`HashMap`/`HashSet`). Internal state maps 128 MIDI notes using fixed-size stack arrays.

### 1.3 Build & Deployment Pipeline

* **Cargo Workspaces:** Manages build targets across sanity tests and main plugin crates.
* **VST3 Bundling:** Handled via `cargo-nih-plug` (`cargo nih-plug bundle <crate_name>`), outputting dynamic link libraries packaged into native `.vst3` directory structures.
* **DAW Deployment Automation:** Managed via Windows Directory Junctions (`mklink /J`) mapping the target bundle directory directly to the system VST3 folder (`C:\Program Files\Common Files\VST3\...`).

---

## 2. Cargo Workspace Architecture

```text
my-vst-project/
├── Cargo.toml                  # Workspace manifest (Resolver v2)
├── Cargo.lock                  # Lockfile
├── mklink.bat                  # VST3 directory junction helper
└── subprojects/
    ├── cli-sanity/             # Sanity Check 1: Command-line binary
    ├── midi-logger-vst/        # Sanity Check 2: Pass-through MIDI VST3 logger
    └── midi-transform-vst/     # Primary Project: Relative Interval Transposer

```

---

## 3. Core Functional Specification: Relative Interval Transposer

### 3.1 High-Level Description

The **Relative Interval Transposer** is a MIDI FX plugin that intercepts incoming MIDI note streams and dynamically transposes pitches based on a user-defined **Root Note** and **Interval Trigger**.

### 3.2 State Machine & Behavioral Specification

#### State 1: Root Assignment (Single Note Input)

* **Trigger:** A single, isolated MIDI `NoteOn` event is received (no other notes held).
* **Behavior:**

1. The plugin sets `Root Note = Incoming Pitch`.
2. The accumulator `Step Count` is reset to `0`.
3. The plugin outputs the note at its original pitch.

#### State 2: Accumulative Interval Transposition (Multi-Note Input)

* **Trigger:** Two notes are struck simultaneously or held together, where one note matches `Root Note` and the second note is `Offset Note`.
* **Behavior:**

1. Calculate base interval:

$$\text{Interval} = \text{Pitch}_{\text{Offset}} - \text{Pitch}_{\text{Root}}$$


2. Increment step count: `Step Count += 1`
3. Calculate output pitch:

$$\text{Output Pitch} = \text{Pitch}_{\text{Offset}} + (\text{Step Count} \times \text{Interval})$$


4. Intercept the raw input notes and emit only the newly calculated `Output Pitch`.

### 3.3 Execution Walkthrough

| Step | User Input | Engine State | Math / Calculation | Plugin MIDI Output |
| --- | --- | --- | --- | --- |
| **1** | Play **C3** alone | `Root = C3`, `Step = 0` | Baseline | **C3** |
| **2** | Play **C3 + E3** | `Interval = +4` (+4 st), `Step = 1` | $E3 + (1 \times 4) = E3$ | **E3** |
| **3** | Re-strike **C3 + E3** | `Interval = +4`, `Step = 2` | $E3 + (2 \times 4) = G\#3$ | **G#3** |
| **4** | Re-strike **C3 + E3** | `Interval = +4`, `Step = 3` | $E3 + (3 \times 4) = C4$ | **C4** |

---

## 4. Edge Cases & Requirements to Resolve

### 4.1 Accumulator Reset Triggers

* **Option A (New Single Note):** Striking any single new note clears `Step Count` to 0 and replaces `Root Note`.
* **Option B (Timeout / All Notes Off):** Releasing all keys resets internal state tracking.

### 4.2 Routing Mode (Interception vs. Polyphony)

* **Mute Original (Interception):** Silences physical keys pressed and outputs *only* the transposed note (Default behavior).
* **Pass-Through (Layered):** Outputs original keys *plus* the transposed interval note.

### 4.3 Directionality & Pitch Bounds

* **Negative Intervals:** If `Offset Note < Root Note` (e.g., Root = C3, Offset = A2, Interval = -3 st), repeated triggers decrement pitch downward ($A2 \rightarrow F\#2 \rightarrow D2$).
* **MIDI Bounds Guard:** Output pitches clamp to valid 7-bit MIDI range ($0 \le \text{Pitch} \le 127$).
