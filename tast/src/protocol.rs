use bitflags::bitflags;

#[derive(Debug, Clone)]
pub(crate) struct EventError;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event: u8 {
        const IMMEDIATE = 0b10000000;
        const PRESSED   = 0b01000000;
        const ID5       = 0b00100000;
        const ID4       = 0b00010000;
        const ID3       = 0b00001000;
        const ID2       = 0b00000100;
        const ID1       = 0b00000010;
        const ID0       = 0b00000001;
        const NONE      = 0b00000000;
        const ID_MASK   = 0b00111111;
    }
}

impl Event {
    pub fn modify_for_pressed(event: Event, pressed: bool) -> Event {
        if pressed {
            event.union(Event::PRESSED)
        } else {
            event
        }
    }
}

pub type EventTime = u32;
pub type TimedEvent = (Event, EventTime);

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct EventChord {
    pub start_at: EventTime,
    pub end_at: EventTime,
}

// We have 6 ID slots and thus support a max of 2⁶ = 64 unique key IDs
pub const EVENTS_LEN: usize = 64;
pub type Events = [EventChord; EVENTS_LEN];

#[cfg(test)]
pub mod tests {
    use super::*;
    pub struct FluidEvent<'a> {
        pub events: &'a mut Events,
    }
    impl<'a> FluidEvent<'a> {
        pub fn press(&mut self, e: Event, time: EventTime) -> &mut FluidEvent<'a> {
            self.events[e.intersection(Event::ID_MASK).bits() as usize].start_at = time;
            self
        }

        pub fn release(&mut self, e: Event, time: EventTime) -> &mut FluidEvent<'a> {
            self.events[e.intersection(Event::ID_MASK).bits() as usize].end_at = time;
            self
        }

        pub(crate) fn new(chords: &'a mut [EventChord; EVENTS_LEN]) -> FluidEvent<'a> {
            FluidEvent { events: chords }
        }
    }
    #[test]
    fn sanity_check_event_api() {
        let id0_id1 = Event::ID0 | Event::ID1;
        assert!(id0_id1.contains(Event::ID0), "0 | 1 should have 0");
        assert!(id0_id1.contains(Event::ID1), "0 | 1 should have 1");
        let pressed_id0_id1 = Event::PRESSED | id0_id1;
        assert!(pressed_id0_id1.contains(Event::PRESSED), "Pressed");
        assert!(pressed_id0_id1.intersection(Event::ID_MASK) == id0_id1, "&");
        assert!(pressed_id0_id1.bits() == 0b01000011, "bits");
    }

    #[cfg(test)]
    mod fluid_event {
        use super::*;
        use crate::protocol::{tests::FluidEvent, EventChord};

        #[test]
        fn home_row() {
            let mut chords = [EventChord {
                start_at: 0,
                end_at: 0,
            }; EVENTS_LEN];
            let mut events = FluidEvent {
                events: &mut chords,
            };
            events.press(Event::ID1, 10).release(Event::ID1, 20);
            assert_eq!(
                events.events[Event::ID1.bits() as usize],
                EventChord {
                    start_at: 10,
                    end_at: 20
                }
            );
        }
    }

    #[cfg(test)]
    mod modify_pressed {
        use super::*;
        #[test]
        fn sixbysix_pass_through_not_pressed() {
            let e = Event::ID0;
            assert_eq!(Event::modify_for_pressed(e, false), e);
        }
        #[test]
        fn sixbysix_set_pressed() {
            let e = Event::ID0;
            assert_eq!(Event::modify_for_pressed(e, true), e | Event::PRESSED);
        }
    }
}
