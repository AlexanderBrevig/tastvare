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
