use usbd_human_interface_device::page::Keyboard;

use crate::{
    layout::six::{LLEFT, LRIGHT},
    protocol::Events,
};

use super::Keymap;

//TODO: pass in QWERTY COLEMAK DVORAK etc
pub struct Microkeys {}

impl Microkeys {}

//TODO: move this or handle dyn size
pub const KEYBOARD_REPORT_SIZE: usize = 6;
impl Keymap<KEYBOARD_REPORT_SIZE> for Microkeys {
    fn generate_report(&self, events: Option<Events>) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]> {
        let mut keys = [Keyboard::NoEventIndicated; KEYBOARD_REPORT_SIZE];
        if events.is_none() {
            return Some(keys);
        }
        let events = events.unwrap();
        let mut id: u8 = 0;
        for (ix, evt) in events.chord.iter().enumerate() {
            if evt.start_at != 0 {
                id |= ix as u8;
            }
        }
        let mut val = 0;
        let mut rev = 0;
        while val < 8 {
            let tmp = id & (1 << val);
            if tmp > 0 {
                rev |= 1 << ((8 - 1) - val);
            }
            val += 1;
        }
        id = rev >> 2;
        keys[0] = Keyboard::from(id);

        match id {
            0b010000 => keys[0] = Keyboard::A,
            0b011000 if events.is_before(LLEFT, LRIGHT) => keys[0] = Keyboard::S,
            0b011000 if events.is_before(LRIGHT, LLEFT) => keys[0] = Keyboard::F,
            0b011000 => keys[0] = Keyboard::D,
            _ => {}
        }
        Some(keys)
    }
}

#[cfg(test)]
mod tests {

    mod home_row {
        use usbd_human_interface_device::page::Keyboard;

        use crate::{
            keymap::{microkeys::Microkeys, Keymap},
            layout::six::{LLEFT, LRIGHT},
            protocol::Events,
        };

        #[test]
        fn home_row_a() {
            let mut events = Events::new();
            events.press(LRIGHT, 10).release(LRIGHT, 20);
            let micro = Microkeys {};
            let report = micro.generate_report(Some(events));
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
            let report = micro.generate_report(Some(events)).unwrap();
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
            let report = micro.generate_report(Some(events)).unwrap();
            assert_eq!(report[0], Keyboard::S);
        }
    }
}
