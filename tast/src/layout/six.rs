use crate::protocol::Event;

pub const KEYS: u8 = 3;
pub const RSHIFT: Event = Event::ID5;
pub const RRIGHT: Event = Event::ID4;
pub const RLEFT: Event = Event::ID3;
pub const LLEFT: Event = Event::ID2;
pub const LRIGHT: Event = Event::ID1;
pub const LSHIFT: Event = Event::ID0;

pub fn event_from(event: Event, pressed: bool) -> Event {
    Event::modify_for_pressed(event, pressed)
}
