use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

use super::Keymap;

pub struct Tinykeys {}

impl Tinykeys {}

//TODO: move this or handle dyn size
pub const KEYBOARD_REPORT_SIZE: usize = 32;
impl Keymap<KEYBOARD_REPORT_SIZE> for Tinykeys {
    fn generate_report(&self, _events: Option<Events>) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]> {
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

        struct FluidEvent<'a> {
            events: &'a mut Events,
        }
        impl<'a> FluidEvent<'a> {
            fn press(&mut self, e: Event) -> &mut FluidEvent<'a> {
                self.events[e.intersection(Event::ID_MASK).bits() as usize].start_at = 10;
                self
            }
            fn release(&mut self, e: Event) -> &mut FluidEvent<'a> {
                self.events[e.intersection(Event::ID_MASK).bits() as usize].end_at = 20;
                self
            }
        }
        #[test]
        fn home_row() {
            let mut chords = [EventChord {
                start_at: 0,
                end_at: 0,
            }; 64];
            let mut events = FluidEvent {
                events: &mut chords,
            };
            //TODO: make an API kind of along these lines
            // let chord = 2;
            // let key = match chord {
            //     0b01000000 => Keyboard::A,
            //     0b0111111100001101 => Keyboard::Keyboard7,
            //     _ => Keyboard::NoEventIndicated,
            // };
            events.press(Event::ID1).release(Event::ID1);
            assert_eq!(
                events.events[2],
                EventChord {
                    start_at: 10,
                    end_at: 20
                }
            );
        }
    }
}
