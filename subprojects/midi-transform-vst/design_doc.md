# System Design Document: Relative Interval Transposer (MIDI FX)

## 1. System Architecture & Toolchain

### 1.1 Overview

The project is a cross-platform (Windows-first) MIDI-in / MIDI-out VST3 audio plugin workspace developed in **Rust** using the **NIH-plug** framework.

### 1.2 Development Environment Configuration

* **Primary OS:** Windows 10/11
* **Target Host / DAW:** Ableton Live (Windows VST3 host)
* **Debugging & Telemetry:**
  * **Host & VST3 Instantiation Troubleshooting:** Inspect Ableton's native application diagnostic log at:
    `C:\Users\austyn\AppData\Roaming\Ableton\Live 11.0.11\Preferences\Log.txt`
    *(Useful for diagnosing issue scenarios where the VST3 fails to scan, crashes on instantiation, or silently rejects being dragged onto a track).*
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
2. The `Current Pitch` is initialized to `Root Note`.
3. The accumulator `Step Count` is reset to `0`.
4. The plugin outputs the note at its original pitch.

#### State 2: Accumulative Interval Transposition (Multi-Note Input)

* **Trigger:** Two notes are struck simultaneously or held together, where one note matches `Root Note` and the second note is `Offset Note`.
* **Behavior:**

1. Calculate interval relative to Root:

$$\text{Interval} = \text{Pitch}_{\text{Offset}} - \text{Pitch}_{\text{Root}}$$

2. Increment step count: `Step Count += 1`
3. Calculate new output pitch additively from `Current Pitch`:

$$\text{Output Pitch} = \text{Current Pitch} + \text{Interval}$$

4. Update state: `Current Pitch = Output Pitch`
5. Intercept the raw input notes and emit only the newly calculated `Output Pitch`.

---

### 3.3 Execution Walkthrough

| Step | User Input | Engine State | Math / Calculation | Plugin MIDI Output |
| --- | --- | --- | --- | --- |
| **1** | Play **C3 (60)** alone | `Root = 60`, `Current = 60`, `Step = 0` | Baseline | **C3 (60)** |
| **2** | Play **C3 + E3 (64)** | `Interval = +4`, `Step = 1` | $60 + 4 = 64$ | **E3 (64)** |
| **3** | Re-strike **E3 (64)** | `Interval = +4`, `Step = 2` | $64 + 4 = 68$ | **G#3 (68)** |
| **4** | Strike **F3 (65)** while holding **C3** | `Interval = +5`, `Step = 3` | $68 + 5 = 73$ | **C#4 (73)** |

---

## 4. Edge Cases & Requirements to Resolve

### 4.1 Accumulator Reset Triggers

The internal step count accumulator and pitch tracking automatically reset under either of the following conditions:

1. **Single Note Trigger (Re-rooting):** Striking any isolated single key resets `Step Count` to `0`, clears active voice mappings, and assigns the new key as `Root Note`.
2. **All Notes Off Trigger:** Releasing all held keys resets internal tracking state and clears all active voice mappings.

### 4.2 Routing Mode (Interception vs. Polyphony)

* **Mute Original (Interception):** Silences physical keys pressed and outputs *only* the transposed note (Default behavior).
* **Pass-Through (Layered):** Outputs original keys *plus* the transposed interval note.

### 4.3 Directionality & Pitch Bounds

* **Negative Intervals:** If `Offset Note < Root Note` (e.g., Root = C3, Offset = A2, Interval = -3 st), repeated triggers decrement pitch downward ($A2 \rightarrow F\#2 \rightarrow D2$).
* **MIDI Bounds Guard:** Output pitches clamp to valid 7-bit MIDI range ($0 \le \text{Pitch} \le 127$).
