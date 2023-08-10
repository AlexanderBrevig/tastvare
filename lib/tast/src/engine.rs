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
        let mut chords = [EventChord::default(); 64];
        if event.0.contains(Event::IMMEDIATE) {
            // IMMEDIATE events are not journaled to the event_log
            let ix = (event.0 & Event::ID_MASK).bits() as usize;
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
                presses += match evnt.contains(Event::PRESSED) {
                    true => 1,
                    false => -1,
                };
            }

            if presses == 0 {
                for (evnt, time) in self.event_log {
                    let ix = (evnt & Event::ID_MASK).bits() as usize;
                    if evnt.contains(Event::PRESSED) {
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
