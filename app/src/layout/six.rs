use rp2040_hal::gpio::{PinId, PullUpInput};

use tast::{
    layout::{six::event_from, Layout},
    protocol::TimedEvent,
};

use super::StatefulGpio;
pub mod microkeys;

pub struct Six<LS, LL, LR, RL, RR, RS>
where
    LS: PinId,
    LL: PinId,
    LR: PinId,
    RL: PinId,
    RR: PinId,
    RS: PinId,
{
    lshift: StatefulGpio<LS, PullUpInput>,
    lleft: StatefulGpio<LL, PullUpInput>,
    lright: StatefulGpio<LR, PullUpInput>,
    rshift: StatefulGpio<RS, PullUpInput>,
    rleft: StatefulGpio<RL, PullUpInput>,
    rright: StatefulGpio<RR, PullUpInput>,
}

impl<LS, LL, LR, RL, RR, RS> Layout for Six<LS, LL, LR, RL, RR, RS>
where
    LS: PinId,
    LL: PinId,
    LR: PinId,
    RL: PinId,
    RR: PinId,
    RS: PinId,
{
    fn get_event(&mut self) -> Option<TimedEvent> {
        use tast::layout::six::{LLEFT, LRIGHT, LSHIFT, RLEFT, RRIGHT, RSHIFT};

        if let Ok(true) = self.lshift.did_change() {
            Some((
                event_from(LSHIFT, self.lshift.pressed),
                self.lshift.changed_at_ms,
            ))
        } else if let Ok(true) = self.lleft.did_change() {
            Some((
                event_from(LLEFT, self.lleft.pressed),
                self.lleft.changed_at_ms,
            ))
        } else if let Ok(true) = self.lright.did_change() {
            Some((
                event_from(LRIGHT, self.lright.pressed),
                self.lright.changed_at_ms,
            ))
        } else if let Ok(true) = self.rleft.did_change() {
            Some((
                event_from(RLEFT, self.rleft.pressed),
                self.rleft.changed_at_ms,
            ))
        } else if let Ok(true) = self.rright.did_change() {
            Some((
                event_from(RRIGHT, self.rright.pressed),
                self.rright.changed_at_ms,
            ))
        } else if let Ok(true) = self.rshift.did_change() {
            Some((
                event_from(RSHIFT, self.rshift.pressed),
                self.rshift.changed_at_ms,
            ))
        } else {
            None
        }
    }
}
