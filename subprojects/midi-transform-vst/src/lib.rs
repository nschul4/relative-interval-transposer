use nih_plug::prelude::*;
use std::num::NonZeroU32;
use std::sync::Arc;

pub struct MidiTransform {
    params: Arc<MidiTransformParams>,
    // Track physical keys currently held down (0..127)
    held_notes: [bool; 128],
    // Maps physical key index -> Active Output Pitch currently sounding on the synth/host
    sounding_pitch: [Option<u8>; 128],
    root_note: Option<u8>,
    current_pitch: Option<u8>,
    step_count: i32,
}

#[derive(Params)]
struct MidiTransformParams {}

impl Default for MidiTransform {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiTransformParams {}),
            held_notes: [false; 128],
            sounding_pitch: [None; 128],
            root_note: None,
            current_pitch: None,
            step_count: 0,
        }
    }
}

impl Plugin for MidiTransform {
    const NAME: &'static str = "Relative Interval Transposer";
    const VENDOR: &'static str = "Neal";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = "0.1.0";

    // Standard stereo audio I/O layout required for Ableton Live track instantiation
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_config: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!(
            "[Transposer] Instantiated {} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn {
                    note,
                    velocity,
                    channel,
                    timing,
                    voice_id,
                } => {
                    let note_idx = (note & 0x7F) as usize;
                    self.held_notes[note_idx] = true;
                    let active_count = self.held_notes.iter().filter(|&&h| h).count();

                    // Single key trigger: Resets accumulator and establishes new baseline root note.
                    if active_count == 1 {
                        // Send NoteOff for any physical key that was somehow still marked sounding
                        for p in 0..128 {
                            if let Some(sounding) = self.sounding_pitch[p].take() {
                                context.send_event(NoteEvent::NoteOff {
                                    timing,
                                    channel,
                                    note: sounding,
                                    velocity: 0.0,
                                    voice_id: None,
                                });
                            }
                        }

                        self.root_note = Some(note);
                        self.current_pitch = Some(note);
                        self.step_count = 0;

                        self.sounding_pitch[note_idx] = Some(note);
                        nih_log!(
                            "[Transposer] Reset Triggered (Single Note) | New Root: {}",
                            note
                        );

                        context.send_event(event);
                    }
                    // Multi-note interval trigger (Root key + Offset key)
                    else if let Some(root) = self.root_note {
                        let root_idx = (root & 0x7F) as usize;
                        if note != root {
                            // Mute baseline root note output if active
                            if let Some(root_pitch) = self.sounding_pitch[root_idx].take() {
                                context.send_event(NoteEvent::NoteOff {
                                    timing,
                                    channel,
                                    note: root_pitch,
                                    velocity: 0.0,
                                    voice_id: None,
                                });
                            }

                            // Mute prior output note assigned to this physical offset key
                            if let Some(prev_pitch) = self.sounding_pitch[note_idx].take() {
                                context.send_event(NoteEvent::NoteOff {
                                    timing,
                                    channel,
                                    note: prev_pitch,
                                    velocity: 0.0,
                                    voice_id: None,
                                });
                            }

                            // Calculate interval relative to root
                            let interval = note as i32 - root as i32;
                            self.step_count += 1;

                            // Add interval directly to current accumulator output pitch
                            let base_pitch = self.current_pitch.unwrap_or(root) as i32;
                            let target_pitch = (base_pitch + interval).clamp(0, 127) as u8;

                            self.current_pitch = Some(target_pitch);

                            nih_log!(
                            "[Transposer] Root: {} | Interval: {:+} | Step: {} -> Out Pitch: {}", 
                            root, interval, self.step_count, target_pitch
                        );

                            self.sounding_pitch[note_idx] = Some(target_pitch);

                            context.send_event(NoteEvent::NoteOn {
                                timing,
                                channel,
                                note: target_pitch,
                                velocity,
                                voice_id,
                            });
                        }
                    }
                }

                NoteEvent::NoteOff {
                    note,
                    channel,
                    timing,
                    ..
                } => {
                    let note_idx = (note & 0x7F) as usize;
                    self.held_notes[note_idx] = false;

                    // Mute whatever pitch this physical key originally triggered
                    if let Some(sounding_note) = self.sounding_pitch[note_idx].take() {
                        context.send_event(NoteEvent::NoteOff {
                            timing,
                            channel,
                            note: sounding_note,
                            velocity: 0.0,
                            voice_id: None,
                        });
                    }

                    // All notes off trigger: Reset tracking state when all keys are released
                    if !self.held_notes.iter().any(|&h| h) {
                        self.root_note = None;
                        self.current_pitch = None;
                        self.step_count = 0;
                        self.sounding_pitch = [None; 128];
                        nih_log!("[Transposer] Reset Triggered (All Notes Released)");
                    }
                }

                _ => {
                    context.send_event(event);
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for MidiTransform {
    const VST3_CLASS_ID: [u8; 16] = [
        0x4B, 0x93, 0xA2, 0x11, 0x58, 0xE1, 0x4F, 0x82, 0xB0, 0x33, 0x61, 0x98, 0xC4, 0xD2, 0xE3,
        0x01,
    ];
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_vst3!(MidiTransform);
