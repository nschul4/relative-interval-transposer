use midi_core::{DomainEvent, Logger};

pub struct TransposerEngine<L: Logger> {
    held_notes: [bool; 128],
    sounding_pitch: [Option<(u8, Option<i32>)>; 128],
    root_note: Option<u8>,
    current_pitch: Option<u8>,
    step_count: i32,
    logger: L,
}

impl<L: Logger> TransposerEngine<L> {
    pub fn new(logger: L) -> Self {
        Self {
            held_notes: [false; 128],
            sounding_pitch: [None; 128],
            root_note: None,
            current_pitch: None,
            step_count: 0,
            logger,
        }
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
                voice_id,
                timing,
            } => {
                let note_idx = (note & 0x7F) as usize;
                self.held_notes[note_idx] = true;

                if self.root_note.is_none() {
                    for p in 0..128 {
                        if let Some((sounding_note, sounding_voice)) = self.sounding_pitch[p].take()
                        {
                            emit(DomainEvent::NoteOff {
                                note: sounding_note,
                                velocity: 0.0,
                                channel,
                                voice_id: sounding_voice,
                                timing,
                            });
                        }
                    }

                    self.root_note = Some(note);
                    self.current_pitch = Some(note);
                    self.step_count = 0;

                    self.sounding_pitch[note_idx] = Some((note, voice_id));
                    self.logger.log(format_args!(
                        "[Transposer] Reset Triggered (New Root) | Root: {}",
                        note
                    ));

                    emit(event);
                } else if let Some(root) = self.root_note {
                    let root_idx = (root & 0x7F) as usize;
                    if note != root {
                        // Mute active root note output
                        if let Some((root_pitch, root_voice)) = self.sounding_pitch[root_idx].take()
                        {
                            emit(DomainEvent::NoteOff {
                                note: root_pitch,
                                velocity: 0.0,
                                channel,
                                voice_id: root_voice,
                                timing,
                            });
                        }

                        // Mute prior note assigned to this physical offset key
                        if let Some((prev_pitch, prev_voice)) = self.sounding_pitch[note_idx].take()
                        {
                            emit(DomainEvent::NoteOff {
                                note: prev_pitch,
                                velocity: 0.0,
                                channel,
                                voice_id: prev_voice,
                                timing,
                            });
                        }

                        // Calculate interval relative to root
                        let interval = note as i32 - root as i32;
                        self.step_count += 1;

                        let base_pitch = self.current_pitch.unwrap_or(root) as i32;
                        let target_pitch = (base_pitch + interval).clamp(0, 127) as u8;

                        self.current_pitch = Some(target_pitch);

                        self.logger.log(format_args!(
                            "[Transposer] Root: {} | Interval: {:+} | Step: {} -> Out Pitch: {}",
                            root, interval, self.step_count, target_pitch
                        ));

                        self.sounding_pitch[note_idx] = Some((target_pitch, voice_id));

                        emit(DomainEvent::NoteOn {
                            note: target_pitch,
                            velocity,
                            channel,
                            voice_id,
                            timing,
                        });
                    }
                }
            }

            DomainEvent::NoteOff {
                note,
                channel,
                timing,
                ..
            } => {
                let note_idx = (note & 0x7F) as usize;
                self.held_notes[note_idx] = false;

                if let Some((sounding_note, sounding_voice)) = self.sounding_pitch[note_idx].take()
                {
                    emit(DomainEvent::NoteOff {
                        note: sounding_note,
                        velocity: 0.0,
                        channel,
                        voice_id: sounding_voice,
                        timing,
                    });
                }

                if !self.held_notes.iter().any(|&h| h) {
                    for p in 0..128 {
                        if let Some((sounding_note, sounding_voice)) = self.sounding_pitch[p].take()
                        {
                            emit(DomainEvent::NoteOff {
                                note: sounding_note,
                                velocity: 0.0,
                                channel,
                                voice_id: sounding_voice,
                                timing,
                            });
                        }
                    }

                    self.root_note = None;
                    self.current_pitch = None;
                    self.step_count = 0;
                    self.logger.log(format_args!(
                        "[Transposer] Reset Triggered (All Notes Released)"
                    ));
                }
            }

            _ => {
                emit(event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use midi_core::NoopLogger;

    #[test]
    fn test_root_note_assignment() {
        let mut engine = TransposerEngine::new(NoopLogger);
        let mut emitted = Vec::new();

        let root_event = DomainEvent::NoteOn {
            note: 60,
            velocity: 0.8,
            channel: 0,
            voice_id: None,
            timing: 0,
        };

        engine.handle_event(root_event, |e| emitted.push(e));

        assert_eq!(emitted.len(), 1);
        assert_eq!(emitted[0], root_event);
        assert_eq!(engine.root_note, Some(60));
        assert_eq!(engine.current_pitch, Some(60));
    }

    #[test]
    fn test_interval_accumulation() {
        let mut engine = TransposerEngine::new(NoopLogger);
        let mut emitted = Vec::new();

        // 1. Trigger C3 (60) -> Root Assignment
        engine.handle_event(
            DomainEvent::NoteOn {
                note: 60,
                velocity: 0.8,
                channel: 0,
                voice_id: None,
                timing: 0,
            },
            |_| {},
        );

        // 2. Play E3 (64) while holding C3 -> +4 semitones (Target 64)
        engine.handle_event(
            DomainEvent::NoteOn {
                note: 64,
                velocity: 0.8,
                channel: 0,
                voice_id: None,
                timing: 0,
            },
            |e| emitted.push(e),
        );

        assert_eq!(emitted.len(), 2);
        assert_eq!(
            emitted[0],
            DomainEvent::NoteOff {
                note: 60,
                velocity: 0.0,
                channel: 0,
                voice_id: None,
                timing: 0,
            }
        );
        assert_eq!(
            emitted[1],
            DomainEvent::NoteOn {
                note: 64,
                velocity: 0.8,
                channel: 0,
                voice_id: None,
                timing: 0,
            }
        );
        assert_eq!(engine.current_pitch, Some(64));
    }
}
