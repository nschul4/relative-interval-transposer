# Design Document: Relative Interval Transposer

## 1. Architecture & Ecosystem

* **Framework:** NIH-plug (Rust)
* **Target Architecture:** VST3 (Windows `x86_64-pc-windows-msvc`)
* **Host Compatibility:** Implements `Instrument` / `Synth` VST3 subcategories and standard 2-channel stereo audio I/O layouts to satisfy track layout negotiation in hosts like Ableton Live.
* **Real-Time Safety:** Zero heap allocations during `process()`. Internal state tracks 128 MIDI notes via fixed-size stack arrays (`[bool; 128]` and `[Option<u8>; 128]`).

---

## 2. Workspace Structure

```text
.
├── Cargo.toml
├── mklink.bat
├── checklink.bat
├── safe-build.sh
└── subprojects/
    ├── cli-sanity/
    │   └── src/main.rs
    ├── midi-logger-vst/
    │   └── src/lib.rs
    └── midi-transform-vst/
        ├── build.rs          # Embeds Git hash & UTC build timestamp
        └── src/lib.rs        # Core state machine logic

```

---

## 3. Core Logic & State Machine

### Internal State

* `held_notes: [bool; 128]` — Physical keys currently held down.
* `sounding_pitch: [Option<u8>; 128]` — Active output pitch mapped to physical key index.
* `root_note: Option<u8>` — Reference root key index.
* `current_pitch: Option<u8>` — Accumulated pitch baseline.
* `step_count: i32` — Incremental interval step counter.

### Behavioral Rules

1. **Single-Note Trigger (Root Assignment):**
* **Condition:** Active `held_notes` count == 1.
* **Behavior:** Mutes any remaining sounding pitches across all slots; sets `root_note = note` and `current_pitch = note`; resets `step_count = 0`; passes through baseline note.


2. **Multi-Note Trigger (Interval Transposition):**
* **Condition:** `root_note` is set and incoming `note != root_note`.
* **Behavior:**
* Sends `NoteOff` for active root note and active output pitch on triggering key slot.
* Calculates interval: $\text{Interval} = \text{Note}_{\text{incoming}} - \text{Root}$
* Increments accumulator: $\text{Target Pitch} = \text{Clamp}_{0..127}(\text{Pitch}_{\text{current}} + \text{Interval})$
* Updates state: `current_pitch = Target Pitch`, `step_count += 1`.
* Emits `NoteOn` for `Target Pitch`.




3. **Release & Reset Triggers:**
* **Single-Key Release:** Sends `NoteOff` for `sounding_pitch` associated with physical key index.
* **All Keys Released:** Sweeps voice array for orphan notes and clears state (`root_note = None`, `current_pitch = None`, `step_count = 0`).



---

## 4. Implementation Details

* **MIDI Range Guard:** Output pitches clamp strictly within 7-bit MIDI bounds ($0 \le p \le 127$).
* **Velocity Handling:** Transposed `NoteOn` events inherit input velocity from triggering offset key; `NoteOff` defaults to `0.0`.
* **Hot-Reloading Helper:** `safe-build.sh` updates compiled `.vst3` target bundles in place to bypass file locking without restarting DAW hosts.
