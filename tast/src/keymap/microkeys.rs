use usbd_human_interface_device::page::Keyboard;

use crate::{
    layout::six::{LLEFT, LRIGHT},
    protocol::Events,
};

use super::{variant_lookup, Keymap, KeymapVariant};

pub struct Microkeys {}

impl Microkeys {}

//TODO: move this or handle dyn size
pub const KEYBOARD_REPORT_SIZE: usize = 6;
impl Keymap<KEYBOARD_REPORT_SIZE> for Microkeys {
    fn generate_report(
        &self,
        events: Option<Events>,
        variant: KeymapVariant,
    ) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]> {
        let mut keys = [Keyboard::NoEventIndicated; KEYBOARD_REPORT_SIZE];
        if events.is_none() {
            return Some(keys);
        }
        let events = events.unwrap();
        let mut id = events.get_id() as u8;

        // Rotate 1011 to 1101
        let mut val: u8 = 0;
        let mut rev: u8 = 0;
        while val < 8 {
            let tmp = id & (1 << val);
            if tmp > 0 {
                rev |= 1 << ((8 - 1) - val);
            }
            val += 1;
        }
        id = rev;
        id >>= 2;

        let handled = match id {
            0b010000 => variant_lookup(variant, Keyboard::A).map(|k| keys[0] = k),
            0b011000 if events.is_before(LLEFT, LRIGHT) => {
                variant_lookup(variant, Keyboard::S).map(|k| keys[0] = k)
            }
            0b011000 if events.is_before(LRIGHT, LLEFT) => {
                variant_lookup(variant, Keyboard::F).map(|k| keys[0] = k)
            }
            0b011000 => variant_lookup(variant, Keyboard::D).map(|k| keys[0] = k),
            0b001000 => variant_lookup(variant, Keyboard::G).map(|k| keys[0] = k),
            _ => {
                keys[0] = Keyboard::from(id);
                None
            }
        };
        handled.map(|_| keys)
    }
}

#[cfg(test)]
mod tests {

    mod home_row {
        use usbd_human_interface_device::page::Keyboard;

        use crate::{
            keymap::{microkeys::Microkeys, qwerty::QWERTY, Keymap},
            layout::six::{LLEFT, LRIGHT},
            protocol::Events,
        };

        #[test]
        fn home_row_a() {
            let mut events = Events::new();
            events.press(LLEFT, 10).release(LLEFT, 20);
            let micro = Microkeys {};
            let report = micro.generate_report(Some(events), QWERTY);
            assert!(report.is_some(), "Report should be Some");
            let report = report.unwrap();
            assert_eq!(report[0], Keyboard::A);
        }

        #[test]
        fn home_row_d() {
            let mut events = Events::new();
            events
                .press(LRIGHT, 10)
                .release(LRIGHT, 20)
                .press(LLEFT, 10)
                .release(LLEFT, 20);
            let micro = Microkeys {};
            let report = micro.generate_report(Some(events), QWERTY).unwrap();
            assert_eq!(report[0], Keyboard::D);
            //TODO: test non-exact D
        }

        #[test]
        fn home_row_s() {
            let mut events = Events::new();
            events
                .press(LRIGHT, 10)
                .release(LRIGHT, 20)
                .press(LLEFT, 20)
                .release(LLEFT, 30);
            let micro = Microkeys {};
            let report = micro.generate_report(Some(events), QWERTY).unwrap();
            assert_eq!(report[0], Keyboard::S);
        }
    }
}
