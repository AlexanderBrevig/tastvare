use crate::protocol::{Event, EventChord, EventTime, Events, TimedEvent};

// #[derive(Debug)]
pub struct Engine {
    event_log: [(Event, EventTime); 64],
    current_ix: usize,
}

impl Engine {
    //handle events, TODO on the form [{start_at, end_at},...] the index is the id
    //allow for hold semantics and order of operation if important for chords
    pub fn handle(&mut self, event: TimedEvent) -> Option<Events> {
        if event.0.contains(Event::IMMEDIATE) {
            // IMMEDIATE events are not journaled to the event_log
            let ix = (event.0 & Event::ID_MASK).bits() as usize;
            let mut chords = [EventChord::default(); 64];
            chords[ix].start_at = event.1;
            chords[ix].end_at = event.1;
            Some(chords)
        } else {
            // Register event to log
            self.event_log[self.current_ix] = event;
            self.current_ix += 1;
            self.current_ix %= 64; //TODO: refactor magic number

            // Check if log is balanced
            let mut presses = 0;
            for (evnt, _) in self.event_log {
                if evnt.bits() == 0 {
                    continue;
                }
                presses += match evnt.contains(Event::PRESSED) {
                    true => 1,
                    false => -1,
                };
            }

            if presses <= 0 {
                let mut chords = [EventChord::default(); 64];
                for (evnt, time) in self.event_log {
                    let ix = (evnt & Event::ID_MASK).bits() as usize;
                    if evnt.contains(Event::PRESSED) {
                        //TODO: do not overwrite?
                        chords[ix].start_at = time;
                    } else {
                        chords[ix].end_at = time;
                    }
                }
                Some(chords)
            } else {
                None
            }
        }
    }

    pub fn new() -> Self {
        Self {
            current_ix: 0,
            event_log: [(Event::NONE, 0); 64],
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        engine::Engine,
        protocol::{Event, EventChord},
    };

    #[test]
    fn handle_none_first() {
        let mut engine = Engine::default();
        assert_eq!(
            engine.handle((Event::PRESSED | Event::ID0, 10)),
            None,
            "engine.handle is None until balanced",
        );
    }
    #[test]
    fn handle_none_unbalanced() {
        let mut engine = Engine::default();
        assert_eq!(
            engine.handle((Event::PRESSED | Event::ID0, 10)),
            None,
            "engine.handle is None until balanced",
        );
        assert_eq!(
            engine.handle((Event::PRESSED | Event::ID1, 10)),
            None,
            "engine.handle is None until balanced",
        );
        assert_eq!(
            engine.handle((Event::PRESSED | Event::ID2, 10)),
            None,
            "engine.handle is None until balanced",
        );
    }
    #[test]
    fn handle_press() {
        let mut engine = Engine::default();
        assert_eq!(
            engine.handle((Event::PRESSED | Event::ID0, 10)),
            None,
            "engine.handle is None until balanced",
        );
        let events = engine.handle((Event::ID0, 12));

        assert_eq!(
            events.unwrap()[Event::ID0.bits() as usize],
            EventChord {
                start_at: 10,
                end_at: 12
            }
        );
    }

    #[test]
    fn handle_two_chord() {
        let mut engine = Engine::default();
        assert_eq!(engine.handle((Event::PRESSED | Event::ID0, 10)), None);
        assert_eq!(engine.handle((Event::PRESSED | Event::ID1, 11)), None);
        assert_eq!(engine.handle((Event::ID0, 12)), None);
        let events = engine.handle((Event::ID1, 14));

        assert_eq!(
            events.unwrap()[Event::ID0.bits() as usize],
            EventChord {
                start_at: 10,
                end_at: 12
            },
            "ID 0"
        );
        assert_eq!(
            events.unwrap()[Event::ID1.bits() as usize],
            EventChord {
                start_at: 11,
                end_at: 14
            },
            "ID 0"
        );
    }
}
