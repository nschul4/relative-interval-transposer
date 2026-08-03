use nih_plug::prelude::*;
use std::sync::Arc;

pub struct MidiTransform {
    params: Arc<MidiTransformParams>,
    // Track physical keys currently held down (0..127)
    held_notes: [bool; 128],
    // Maps physical key index -> Active Output Pitch currently sounding on the synth/host
    sounding_pitch: [Option<u8>; 128],
    root_note: Option<u8>,
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

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, channel, timing, voice_id } => {
                    let note_idx = (note & 0x7F) as usize;
                    self.held_notes[note_idx] = true;
                    let active_count = self.held_notes.iter().filter(|&&h| h).count();

                    // 1. Single note pressed: Establish Root and pass through note as-is
                    if active_count == 1 {
                        self.root_note = Some(note);
                        self.step_count = 0;

                        self.sounding_pitch[note_idx] = Some(note);
                        context.send_event(event);
                    } 
                    // 2. Multi-note interval trigger (Root + Offset Key)
                    else if let Some(root) = self.root_note {
                        let root_idx = (root & 0x7F) as usize;
                        if note != root {
                            // Silence the Root note if it is currently sounding
                            if let Some(root_pitch) = self.sounding_pitch[root_idx].take() {
                                context.send_event(NoteEvent::NoteOff {
                                    timing,
                                    channel,
                                    note: root_pitch,
                                    velocity: 0.0,
                                    voice_id: None,
                                });
                            }

                            // Silence any prior transposed note triggered by re-striking this key
                            if let Some(prev_pitch) = self.sounding_pitch[note_idx].take() {
                                context.send_event(NoteEvent::NoteOff {
                                    timing,
                                    channel,
                                    note: prev_pitch,
                                    velocity: 0.0,
                                    voice_id: None,
                                });
                            }

                            // Calculate base interval relative to root
                            let interval = note as i32 - root as i32;
                            self.step_count += 1;
                            // Calculate transposed target pitch
                            let target_pitch = (root as i32 + (self.step_count * interval)).clamp(0, 127) as u8;
                            
                            nih_log!(
                                "[Transposer] Root: {} | Interval: {} | Step: {} -> Out Pitch: {}", 
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

                NoteEvent::NoteOff { note, channel, timing, .. } => {
                    let note_idx = (note & 0x7F) as usize;
                    self.held_notes[note_idx] = false;

                    // Send NoteOff for whatever target pitch this physical key originally triggered
                    if let Some(sounding_note) = self.sounding_pitch[note_idx].take() {
                        context.send_event(NoteEvent::NoteOff {
                            timing,
                            channel,
                            note: sounding_note,
                            velocity: 0.0,
                            voice_id: None,
                        });
                    }

                    // Reset root tracking and state if all keys have been released
                    if !self.held_notes.iter().any(|&h| h) {
                        self.root_note = None;
                        self.step_count = 0;
                        self.sounding_pitch = [None; 128];
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
        0x4B, 0x93, 0xA2, 0x11, 0x58, 0xE1, 0x4F, 0x82,
        0xB0, 0x33, 0x61, 0x98, 0xC4, 0xD2, 0xE3, 0x01,
    ];
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
    ];
}

nih_export_vst3!(MidiTransform);