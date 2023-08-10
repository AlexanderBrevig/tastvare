use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

use super::Keymap;

pub struct Ergonomisk {}

impl Ergonomisk {}

pub const KEYBOARD_REPORT_SIZE: usize = 34;
impl Keymap<KEYBOARD_REPORT_SIZE> for Ergonomisk {
    fn generate_report(&self, events: Events) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]> {
        let mut keys = [None; KEYBOARD_REPORT_SIZE];
        keys[0] = Some(Keyboard::A);
        keys
    }
}
