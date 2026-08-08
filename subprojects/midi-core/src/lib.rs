use nih_plug::prelude::*;
use std::num::NonZeroU32;

/// Standard logger trait for plug-in engines
pub trait Logger {
    fn log(&self, msg: std::fmt::Arguments);
}

/// No-op logger implementation for tests and default instantiation
pub struct NoopLogger;

impl Logger for NoopLogger {
    #[inline(always)]
    fn log(&self, _msg: std::fmt::Arguments) {}
}

/// Unified domain event representation across all MIDI engine crates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DomainEvent {
    NoteOn {
        note: u8,
        velocity: f32,
        channel: u8,
        voice_id: Option<i32>,
        timing: u32,
    },
    NoteOff {
        note: u8,
        velocity: f32,
        channel: u8,
        voice_id: Option<i32>,
        timing: u32,
    },
    MidiCC {
        cc: u8,
        value: f32,
        channel: u8,
        timing: u32,
    },
}

/// Conversion from NIH-Plug event to DomainEvent
pub fn nih_to_domain(event: &NoteEvent<()>) -> Option<DomainEvent> {
    match *event {
        NoteEvent::NoteOn {
            note,
            velocity,
            channel,
            timing,
            voice_id,
        } => Some(DomainEvent::NoteOn {
            note,
            velocity,
            channel,
            voice_id,
            timing,
        }),
        NoteEvent::NoteOff {
            note,
            velocity,
            channel,
            timing,
            voice_id,
        } => Some(DomainEvent::NoteOff {
            note,
            velocity,
            channel,
            voice_id,
            timing,
        }),
        NoteEvent::MidiCC {
            cc,
            value,
            channel,
            timing,
        } => Some(DomainEvent::MidiCC {
            cc,
            value,
            channel,
            timing,
        }),
        _ => None,
    }
}

/// Conversion from DomainEvent back to NIH-Plug event
pub fn domain_to_nih(event: DomainEvent) -> Option<NoteEvent<()>> {
    match event {
        DomainEvent::NoteOn {
            note,
            velocity,
            channel,
            voice_id,
            timing,
        } => Some(NoteEvent::NoteOn {
            timing,
            channel,
            note,
            velocity,
            voice_id,
        }),
        DomainEvent::NoteOff {
            note,
            velocity,
            channel,
            voice_id,
            timing,
        } => Some(NoteEvent::NoteOff {
            timing,
            channel,
            note,
            velocity,
            voice_id,
        }),
        DomainEvent::MidiCC {
            cc,
            value,
            channel,
            timing,
        } => Some(NoteEvent::MidiCC {
            timing,
            channel,
            cc,
            value,
        }),
    }
}

/// Shared audio I/O layouts to satisfy host channel negotiation
pub const DEFAULT_INSTRUMENT_LAYOUTS: &[AudioIOLayout] = &[
    AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    },
    AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    },
];

/// Shared MIDI configuration options
pub const DEFAULT_MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
pub const DEFAULT_MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
