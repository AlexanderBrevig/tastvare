use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

pub mod tinykeys;

pub trait Keymap<const KEYBOARD_REPORT_SIZE: usize> {
    fn generate_report(&self, events: Option<Events>) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]>;
}
