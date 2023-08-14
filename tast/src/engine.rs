use crate::{
    keymap::Keymap,
    layout::Layout,
    protocol::{Event, EventChord, EventTime, Events, TimedEvent},
    report::UsbReporter,
    serial::EventSource,
};

// #[derive(Debug)]
pub struct Engine<const KEYBOARD_REPORT_SIZE: usize, L, K, USB, ES>
where
    L: Layout,
    K: Keymap<KEYBOARD_REPORT_SIZE>,
    USB: UsbReporter,
    ES: EventSource,
{
    event_log: [(Event, EventTime); 64],
    current_ix: usize,
    layout: L,
    keymap: K,
    serial: ES,
    report: USB,
}

impl<const KRS: usize, L, K, USB, ES> Engine<KRS, L, K, USB, ES>
where
    L: Layout,
    K: Keymap<KRS>,
    USB: UsbReporter,
    ES: EventSource,
{
    pub fn poll(&mut self) {
        self.report.poll();
    }
    pub fn process(&mut self) {
        //TODO: place in a Tastvare::App
        while let Some(timed_event) = self.serial.get_event() {
            let event = self.process_timed_event(timed_event);
            let report = self.keymap.generate_report(event);
            self.report.write_report(report).ok();
        }

        while let Some(timed_event) = self.layout.get_event() {
            self.serial.send_event(timed_event.0);
            let event = self.process_timed_event(timed_event);
            let report = self.keymap.generate_report(event);
            self.report.write_report(report).ok();
        }
        self.report.tick().ok();
    }

    //handle events, TODO on the form [{start_at, end_at},...] the index is the id
    //allow for hold semantics and order of operation if important for chords
    fn process_timed_event(&mut self, event: TimedEvent) -> Option<Events> {
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

    pub fn new(layout: L, keymap: K, report: USB, serial: ES) -> Self {
        Self {
            current_ix: 0,
            event_log: [(Event::NONE, 0); 64],
            layout,
            keymap,
            report,
            serial,
        }
    }
}

#[cfg(test)]
mod tests {
    use usbd_human_interface_device::page::Keyboard;

    use super::*;
    use crate::{
        keymap::tests::TestKeymap,
        layout::tests::TestLayout,
        protocol::{Event, EventChord},
        serial::tests::TestEventSource,
    };
    fn engine(
        event: Option<TimedEvent>,
        events: Option<[Keyboard; 64]>,
    ) -> Engine<64, TestLayout, TestKeymap, TestEventSource> {
        Engine {
            event_log: [(Event::NONE, 0); 64],
            current_ix: 0,
            layout: TestLayout { event },
            keymap: TestKeymap { events },
            serial: TestEventSource { event },
        }
    }
    #[test]
    fn handle_none_first() {
        let mut engine = engine(None, None);
        assert_eq!(
            engine.process_timed_event((Event::PRESSED | Event::ID0, 10)),
            None,
            "engine.process_timed_event is None until balanced",
        );
    }
    #[test]
    fn handle_none_unbalanced() {
        let mut engine = engine(None, None);
        assert_eq!(
            engine.process_timed_event((Event::PRESSED | Event::ID0, 10)),
            None,
            "engine.process_timed_event is None until balanced",
        );
        assert_eq!(
            engine.process_timed_event((Event::PRESSED | Event::ID1, 10)),
            None,
            "engine.process_timed_event is None until balanced",
        );
        assert_eq!(
            engine.process_timed_event((Event::PRESSED | Event::ID2, 10)),
            None,
            "engine.process_timed_event is None until balanced",
        );
    }
    #[test]
    fn handle_press() {
        let mut engine = engine(None, None);
        assert_eq!(
            engine.process_timed_event((Event::PRESSED | Event::ID0, 10)),
            None,
            "engine.process_timed_event is None until balanced",
        );
        let events = engine.process_timed_event((Event::ID0, 12));

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
        let mut engine = engine(None, None);
        assert_eq!(
            engine.process_timed_event((Event::PRESSED | Event::ID0, 10)),
            None
        );
        assert_eq!(
            engine.process_timed_event((Event::PRESSED | Event::ID1, 11)),
            None
        );
        assert_eq!(engine.process_timed_event((Event::ID0, 12)), None);
        let events = engine.process_timed_event((Event::ID1, 14));
        /// NO MORE ERRORS WOHOOO
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
