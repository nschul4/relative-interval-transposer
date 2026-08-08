use midi_core::{DomainEvent, Logger};

pub struct LoggerEngine<L: Logger> {
    logger: L,
}

impl<L: Logger> LoggerEngine<L> {
    pub fn new(logger: L) -> Self {
        Self { logger }
    }

    pub fn handle_event<F>(&mut self, event: DomainEvent, mut emit: F)
    where
        F: FnMut(DomainEvent),
    {
        match event {
            DomainEvent::NoteOn {
                note,
                velocity,
                channel,
                ..
            } => {
                self.logger.log(format_args!(
                    "[MIDI Logger] Note On  | Ch: {} | Note: {} | Vel: {}",
                    channel, note, velocity
                ));
            }
            DomainEvent::NoteOff { note, channel, .. } => {
                self.logger.log(format_args!(
                    "[MIDI Logger] Note Off | Ch: {} | Note: {}",
                    channel, note
                ));
            }
            DomainEvent::MidiCC {
                cc, value, channel, ..
            } => {
                self.logger.log(format_args!(
                    "[MIDI In] CC #{:<3}   | Ch: {} | Val: {}",
                    cc, channel, value
                ));
            }
        }

        // Passthrough event untouched
        emit(event);
    }
}
