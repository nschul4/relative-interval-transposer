# Relative Interval Transposer (MIDI FX)

A cross-platform (Windows-first) VST3 MIDI plugin workspace built in **Rust** using the **NIH-plug** framework.

This project intercepts incoming MIDI note streams and dynamically transposes pitches based on a user-defined **Root Note** and **Interval Trigger**. Holding a root key while striking an offset key calculates the musical interval and steps the transposition outward on subsequent hits, providing an expressive tool for live performance and algorithmic composition.

---

## Workspace Layout

The repository is structured as a **Cargo Workspace** containing both sanity-check subprojects and the primary MIDI plugin:

```text
.
├── Cargo.toml                  # Workspace manifest[cite: 2]
├── mklink.bat                  # Automated VST3 directory junction helper[cite: 2]
└── subprojects/
    ├── cli-sanity/             # Console app to verify Rust toolchain setup[cite: 2]
    ├── midi-logger-vst/        # Diagnostic VST3 pass-through MIDI logger[cite: 2]
    └── midi-transform-vst/     # Primary Relative Interval Transposer plugin[cite: 2]

```

---

## Prerequisites

* **Rust Toolchain:** Install [Rustup](https://rustup.rs/) (targets `x86_64-pc-windows-msvc`).


* **Terminal Shell:** Cygwin (Bash) or Windows Command Prompt / PowerShell.


* **DAW Host:** Ableton Live (or any VST3-compatible host).


* **NIH-Plug Bundler:** Install the bundler tool once globally:


```bash
cargo install cargo-nih-plug

```


* **(Optional) Microsoft DebugView:** To monitor live `nih_log!` output in real-time (`Dbgview.exe`).



---

## Building the Workspace

### 1. Verify Toolchain (CLI Sanity Check)

Run the standard console binary from your terminal to verify your Rust environment:

```bash
cargo run -p cli-sanity

```

### 2. Build & Bundle VST3 Plugins

Compile and package the `.vst3` bundles for both plugins:

```bash
# Build the MIDI Logger
cargo nih-plug bundle midi-logger-vst

# Build the Relative Interval Transposer
cargo nih-plug bundle midi-transform-vst

```

The compiled bundles will be generated under `target/bundled/`:

* `target/bundled/midi-logger-vst.vst3`

* `target/bundled/midi-transform-vst.vst3`


---

## Linking to Ableton / VST3 Directory

To avoid manually copying `.vst3` files after every build, link the build directory to the system VST3 folder (`C:\Program Files\Common Files\VST3\`):

1. Open an **Elevated Command Prompt** (Right-click -> *Run as Administrator*) or an elevated Cygwin session.


2. Execute the included batch script:


```cmd
mklink.bat

```



This creates directory junctions pointing directly to your compiled build artifacts.

---

## Testing in Ableton Live

1. Launch **Ableton Live**.
2. Navigate to **Preferences** $\rightarrow$ **Plug-ins** and click **Rescan**.


3. Insert `Relative Interval Transposer` onto a **MIDI Track**.


4. Route the MIDI output of this track into a synth instrument track (or place an instrument directly after the MIDI plugin).


5. **Play:**
* Strike **C3** alone $\rightarrow$ Passes through baseline **C3**.


* Hold **C3** and hit **E3** (+4 semitones) $\rightarrow$ Outputs **E3**.


* Re-strike **E3** while still holding **C3** $\rightarrow$ Step count increments and transposes pitch to **G#3** ($E3 + 4\text{st}$).


* Release all keys $\rightarrow$ State machine resets.





### Viewing Live Logs

Run **DebugView** (`Dbgview.exe`) with *Capture Win32* enabled to inspect real-time log messages (`[Transposer] Root: ... | Out Pitch: ...`) as you play.