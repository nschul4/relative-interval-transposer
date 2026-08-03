use nih_plug::prelude::*;
use std::sync::Arc;

pub struct MidiLogger {
    params: Arc<MidiLoggerParams>,
}

#[derive(Params)]
struct MidiLoggerParams {}

impl Default for MidiLogger {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiLoggerParams {}),
        }
    }
}

impl Plugin for MidiLogger {
    const NAME: &'static str = "MIDI Logger";
    const VENDOR: &'static str = "Sanity Check";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = "0.1.0";

    // Inform host that this plugin accepts and passes MIDI
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
        _buffer: &mut Buffer, // Added underscore to signal unused variable
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Loop over incoming MIDI events in the current audio block
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, channel, .. } => {
                    nih_log!("[MIDI In] Note On  | Ch: {} | Note: {} | Vel: {}", channel, note, velocity);
                }
                NoteEvent::NoteOff { note, channel, .. } => {
                    nih_log!("[MIDI In] Note Off | Ch: {} | Note: {}", channel, note);
                }
                NoteEvent::MidiCC { cc, value, channel, .. } => {
                    nih_log!("[MIDI In] CC #{:<3}   | Ch: {} | Val: {}", cc, channel, value);
                }
                _ => {}
            }

            // Passthrough MIDI event to output buffer
            context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for MidiLogger {
    const VST3_CLASS_ID: [u8; 16] = *b"MidiLoggerSanity";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
    ];
}

nih_export_vst3!(MidiLogger);