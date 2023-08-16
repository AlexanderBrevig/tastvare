use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

pub mod microkeys;
pub mod tinykeys;

pub trait Keymap<const KEYBOARD_REPORT_SIZE: usize> {
    fn generate_report(&self, events: Option<Events>) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]>;
}

#[cfg(test)]
pub mod tests {
    use super::Keymap;

    pub struct TestKeymap {
        pub(crate) events: Option<[usbd_human_interface_device::page::Keyboard; 64]>,
    }
    impl Keymap<64> for TestKeymap {
        fn generate_report(
            &self,
            _events: Option<crate::protocol::Events>,
        ) -> Option<[usbd_human_interface_device::page::Keyboard; 64]> {
            self.events
        }
    }
}
