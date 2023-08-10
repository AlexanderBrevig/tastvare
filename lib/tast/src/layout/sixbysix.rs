use crate::protocol::Event;

pub const THUMB2: u8 = Event::ID5.bits();
pub const THUMB1: u8 = Event::ID4.bits();
pub const INDEX: u8 = Event::ID3.bits();
pub const MIDDLE: u8 = Event::ID2.bits();
pub const RING: u8 = Event::ID1.bits();
pub const PINKY: u8 = Event::ID0.bits();

#[cfg(feature = "left")]
pub fn event_from(mask: u8, pressed: bool) -> Option<Event> {
    let mut event = Event::from_bits(mask)?;
    if pressed {
        event |= Event::PRESSED;
    }
    Some(event)
}

#[cfg(feature = "right")]
pub fn event_from(mask: u8, pressed: bool) -> Option<Event> {
    // The right side flips all the ID bits
    let mut event = !Event::from_bits(mask)?;
    if pressed {
        event |= Event::PRESSED;
    }
    Some(event)
}
