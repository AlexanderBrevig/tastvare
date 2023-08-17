use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

use super::{Keymap, KeymapVariant};

pub struct Tinykeys {}

impl Tinykeys {}

//TODO: move this or handle dyn size
pub const KEYBOARD_REPORT_SIZE: usize = 32;
impl Keymap<KEYBOARD_REPORT_SIZE> for Tinykeys {
    fn generate_report(
        &self,
        _events: Option<Events>,
        _variant: KeymapVariant,
    ) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]> {
        let mut keys = [Keyboard::NoEventIndicated; KEYBOARD_REPORT_SIZE];
        keys[0] = Keyboard::A;
        todo!();
        // Some(keys)
    }
}

#[cfg(test)]
mod tests {

    mod no_mod_layer {

        use crate::protocol::{Event, EventChord, Events};

        #[test]
        fn home_row() {
            let mut events = Events::new();
            events.press(Event::ID1, 10).release(Event::ID1, 20);
            assert_eq!(
                events.chord[Event::ID1.bits() as usize],
                EventChord {
                    start_at: 10,
                    end_at: 20
                }
            );
        }
    }
}
