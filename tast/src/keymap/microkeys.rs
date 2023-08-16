use usbd_human_interface_device::page::Keyboard;

use crate::protocol::Events;

use super::Keymap;

//TODO: pass in QWERTY COLEMAK DVORAK etc
pub struct Microkeys {}

impl Microkeys {}

//TODO: move this or handle dyn size
pub const KEYBOARD_REPORT_SIZE: usize = 6;
impl Keymap<KEYBOARD_REPORT_SIZE> for Microkeys {
    fn generate_report(&self, _events: Option<Events>) -> Option<[Keyboard; KEYBOARD_REPORT_SIZE]> {
        let mut keys = [Keyboard::NoEventIndicated; KEYBOARD_REPORT_SIZE];
        keys[0] = Keyboard::A;
        Some(keys)
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{EventChord, EVENTS_LEN};

    const NO_CHORD: [EventChord; EVENTS_LEN] = [EventChord {
        start_at: 0,
        end_at: 0,
    }; EVENTS_LEN];
    mod home_row {

        use usbd_human_interface_device::page::Keyboard;

        use super::*;
        use crate::{
            keymap::{microkeys::Microkeys, Keymap},
            layout::six::{INDEX, MIDDLE},
            protocol::tests::FluidEvent,
        };

        #[test]
        fn home_row_a() {
            let mut chords = NO_CHORD;
            let mut events = FluidEvent::new(&mut chords);
            events.press(MIDDLE, 10).release(MIDDLE, 20);
            let micro = Microkeys {};
            let report = micro.generate_report(Some(events.events.to_owned()));
            assert!(report.is_some(), "Report should be Some");
            let report = report.unwrap();
            assert_eq!(report[0], Keyboard::A);
        }
        #[test]
        fn home_row_d() {
            let mut chords = NO_CHORD;
            let mut events = FluidEvent::new(&mut chords);
            events
                .press(MIDDLE, 10)
                .release(MIDDLE, 20)
                .press(INDEX, 10)
                .release(INDEX, 20);
            let micro = Microkeys {};
            let report = micro
                .generate_report(Some(events.events.to_owned()))
                .unwrap();
            assert_eq!(report[0], Keyboard::D);
        }
    }
}
