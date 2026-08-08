mod engine;

use engine::TransposerEngine;
use midi_core::{
    domain_to_nih, nih_to_domain, Logger, DEFAULT_INSTRUMENT_LAYOUTS, DEFAULT_MIDI_INPUT,
    DEFAULT_MIDI_OUTPUT,
};
use nih_plug::prelude::*;
use std::sync::Arc;

struct NihLogger;
impl Logger for NihLogger {
    #[inline(always)]
    fn log(&self, _message: std::fmt::Arguments) {
        nih_log!("{}", _message);
    }
}

pub struct MidiTransform {
    params: Arc<MidiTransformParams>,
    engine: TransposerEngine<NihLogger>,
}

#[derive(Params)]
struct MidiTransformParams {}

impl Default for MidiTransform {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiTransformParams {}),
            engine: TransposerEngine::new(NihLogger),
        }
    }
}

impl Plugin for MidiTransform {
    const NAME: &'static str = "Relative Interval Transposer";
    const VENDOR: &'static str = "Neal";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = "0.1.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = DEFAULT_INSTRUMENT_LAYOUTS;

    const MIDI_INPUT: MidiConfig = DEFAULT_MIDI_INPUT;
    const MIDI_OUTPUT: MidiConfig = DEFAULT_MIDI_OUTPUT;

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
        NihLogger.log(format_args!(
            "[Transposer] Instantiated {} v{} (Build: {} | Time: {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("BUILD_GIT_HASH"),
            env!("BUILD_TIMESTAMP")
        ));
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            if let Some(domain_evt) = nih_to_domain(&event) {
                self.engine.handle_event(domain_evt, |out_domain_evt| {
                    if let Some(out_nih_evt) = domain_to_nih(out_domain_evt) {
                        context.send_event(out_nih_evt);
                    }
                });
            } else {
                context.send_event(event);
            }
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for MidiTransform {
    const VST3_CLASS_ID: [u8; 16] = *b"MidiRelativeTran";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_vst3!(MidiTransform);
