use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

use super::{variant_lookup, Keymap, KeymapVariant};

pub struct Ergonomisk {}

impl Ergonomisk {}

pub const KEYBOARD_REPORT_SIZE: usize = 34;
impl Keymap<KEYBOARD_REPORT_SIZE> for Ergonomisk {
    fn generate_report(
        &self,
        events: Option<Events>,
        variant: KeymapVariant,
    ) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]> {
        let mut keys = [Keyboard::NoEventIndicated; KEYBOARD_REPORT_SIZE];
        if let Some(events) = events {
            for (ix, chord) in events.chord.iter().enumerate() {
                if chord.start_at != 0 {
                    // TODO: separate left / right scan by ID5 bit
                    let key = match ix {
                        0 => variant_lookup(variant, Keyboard::Q),
                        1 => variant_lookup(variant, Keyboard::W),
                        _ => None,
                    };
                    if let Some(key) = key {
                        keys[ix] = key;
                    }
                }
            }
        }
        Some(keys)
    }
}
