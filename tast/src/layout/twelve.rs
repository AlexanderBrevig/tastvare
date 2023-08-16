use crate::protocol::Event;

pub const THUMB2: Event = Event::ID5;
pub const THUMB1: Event = Event::ID4;
pub const INDEX: Event = Event::ID3;
pub const MIDDLE: Event = Event::ID2;
pub const RING: Event = Event::ID1;
pub const PINKY: Event = Event::ID0;

//TODO: is this even needed?

#[cfg(feature = "left")]
pub fn event_from(event: Event, pressed: bool) -> Event {
    Event::modify_for_pressed(event, pressed)
}

#[cfg(feature = "right")]
pub fn event_from(event: Event, pressed: bool) -> Event {
    // The right side flips all the ID bits
    Event::modify_for_pressed(!event, pressed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sixbysix_pass_through_not_pressed() {
        assert_eq!(Event::modify_for_pressed(INDEX, false), INDEX);
    }
    #[test]
    fn sixbysix_set_pressed() {
        assert_eq!(
            Event::modify_for_pressed(INDEX, true),
            INDEX | Event::PRESSED
        );
    }
}
