use bitflags::bitflags;

#[derive(Debug, Clone)]
pub(crate) struct EventError;

pub type EventTime = u32;

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

pub type TimedEvent = (Event, EventTime);

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct EventChord {
    pub start_at: EventTime,
    pub end_at: EventTime,
}

// We have 6 ID slots and thus support a max of 2⁶ = 64 unique key IDs
pub type Events = [EventChord; 64];

#[cfg(test)]
mod tests {
    use super::*;
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
}
