use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

pub const KEYMAP_VARIANT_LENGTH: usize = 26;

pub mod colemak;
pub mod ergonomisk;
pub mod microkeys;
pub mod qwerty;
pub mod tinykeys;

pub type KeymapVariant = [Keyboard; KEYMAP_VARIANT_LENGTH];

type KeymapReport<const KEYBOARD_REPORT_SIZE: usize> = [Keyboard; KEYBOARD_REPORT_SIZE];

pub trait Keymap<const KEYBOARD_REPORT_SIZE: usize> {
    fn generate_report(
        &self,
        events: Option<Events>,
        variant: KeymapVariant,
    ) -> Option<KeymapReport<KEYBOARD_REPORT_SIZE>>;
}

pub fn variant_lookup(variant: KeymapVariant, key: Keyboard) -> Option<Keyboard> {
    if key as usize > KEYMAP_VARIANT_LENGTH {
        None
    } else {
        Some(variant[key as usize - Keyboard::A as usize])
    }
}

#[cfg(test)]
pub mod tests {
    use usbd_human_interface_device::page::Keyboard;

    use super::{variant_lookup, Keymap, KeymapVariant};

    pub struct TestKeymap {
        pub input: Keyboard,
        pub(crate) events: Option<[usbd_human_interface_device::page::Keyboard; 64]>,
    }
    impl Keymap<64> for TestKeymap {
        fn generate_report(
            &self,
            _events: Option<crate::protocol::Events>,
            variant: KeymapVariant,
        ) -> Option<[usbd_human_interface_device::page::Keyboard; 64]> {
            let mut evts = [Keyboard::NoEventIndicated; 64];
            evts[0] = variant_lookup(variant, self.input).unwrap();
            Some(evts)
        }
    }
    mod variant_lookup {
        use usbd_human_interface_device::page::Keyboard;

        use crate::keymap::{qwerty::QWERTY, tests::variant_lookup};

        #[test]
        fn a_is_a() {
            assert_eq!(variant_lookup(QWERTY, Keyboard::A), Some(Keyboard::A));
        }

        #[test]
        fn out_of_bounds_is_none() {
            assert_eq!(variant_lookup(QWERTY, Keyboard::F1), None);
        }
    }
    mod qwerty_keymap_variant {
        use super::*;
        use crate::keymap::qwerty::QWERTY;

        #[test]
        pub fn check_a() {
            let k = TestKeymap {
                events: Some([Keyboard::NoEventIndicated; 64]),
                input: Keyboard::A,
            };
            assert_eq!(k.generate_report(None, QWERTY).unwrap()[0], Keyboard::A);
        }

        #[test]
        pub fn check_d() {
            let k = TestKeymap {
                events: Some([Keyboard::NoEventIndicated; 64]),
                input: Keyboard::D,
            };
            assert_eq!(k.generate_report(None, QWERTY).unwrap()[0], Keyboard::D);
        }
    }
    mod colemak_keymap_variant {
        use super::*;
        use crate::keymap::colemak::COLEMAK_DH;

        #[test]
        pub fn check_a() {
            let k = TestKeymap {
                events: Some([Keyboard::NoEventIndicated; 64]),
                input: Keyboard::A,
            };
            assert_eq!(k.generate_report(None, COLEMAK_DH).unwrap()[0], Keyboard::A);
        }

        #[test]
        pub fn check_d() {
            let k = TestKeymap {
                events: Some([Keyboard::NoEventIndicated; 64]),
                input: Keyboard::D,
            };
            assert_eq!(k.generate_report(None, COLEMAK_DH).unwrap()[0], Keyboard::D);
        }
    }
}
