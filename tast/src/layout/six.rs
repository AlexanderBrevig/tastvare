use crate::protocol::Event;

pub const KEYS: u8 = 3;
pub const INDEX: Event = Event::ID2;
pub const MIDDLE: Event = Event::ID1;
pub const RING: Event = Event::ID0;

#[cfg(feature = "left")]
pub fn event_from(event: Event, pressed: bool) -> Event {
    Event::modify_for_pressed(event, pressed)
}

#[cfg(feature = "right")]
pub fn event_from(event: Event, pressed: bool) -> Event {
    // The right side moves all bits up 3 pos to ID 3 4 5
    Event::modify_for_pressed(event << KEYS, pressed)
}
