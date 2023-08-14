use crate::protocol::Event;

pub const THUMB2: u8 = Event::ID5.bits();
pub const THUMB1: u8 = Event::ID4.bits();
pub const INDEX: u8 = Event::ID3.bits();
pub const MIDDLE: u8 = Event::ID2.bits();
pub const RING: u8 = Event::ID1.bits();
pub const PINKY: u8 = Event::ID0.bits();

//TODO: is this even needed?

#[cfg(feature = "left")]
pub fn event_from(mask: u8, pressed: bool) -> Option<Event> {
    event_from_impl(Event::from_bits(mask), pressed)
}

#[cfg(feature = "right")]
pub fn event_from(mask: u8, pressed: bool) -> Option<Event> {
    // The right side flips all the ID bits
    event_from_impl(!Event::from_bits(mask), pressed)
}

fn event_from_impl(event: Option<Event>, pressed: bool) -> Option<Event> {
    match event {
        None => None,
        Some(event) => {
            if pressed {
                Some(event.union(Event::PRESSED))
            } else {
                Some(event)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sixbysix_none_gives_none() {
        assert_eq!(event_from_impl(None, false), None);
    }
    #[test]
    fn sixbysix_pass_through_not_pressed() {
        let e = Event::ID0;
        assert_eq!(event_from_impl(Some(e), false), Some(e));
    }
    #[test]
    fn sixbysix_set_pressed() {
        let e = Event::ID0;
        assert_eq!(event_from_impl(Some(e), true), Some(e | Event::PRESSED));
    }
}
