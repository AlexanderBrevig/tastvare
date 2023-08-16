use rp2040_hal::gpio::{PinId, PullUpInput};

use tast::{
    layout::{
        six::event_from,
        six::{INDEX, MIDDLE, PINKY, RING, THUMB1, THUMB2},
        Layout,
    },
    protocol::TimedEvent,
};

use super::StatefulGpio;
pub mod tinykeys;

pub struct Six<R, M, I>
where
    R: PinId,
    M: PinId,
    I: PinId,
{
    ring: StatefulGpio<R, PullUpInput>,
    middle: StatefulGpio<M, PullUpInput>,
    index: StatefulGpio<I, PullUpInput>,
}

impl<R, M, I> Layout for Six<M, I, T1>
where
    R: PinId,
    M: PinId,
    I: PinId,
{
    fn get_event(&mut self) -> Option<TimedEvent> {
        if let Ok(true) = self.ring.did_change() {
            event_from(RING, self.ring.pressed).map(|e| (e, self.ring.changed_at_ms))
        } else if let Ok(true) = self.middle.did_change() {
            event_from(MIDDLE, self.middle.pressed).map(|e| (e, self.middle.changed_at_ms))
        } else if let Ok(true) = self.index.did_change() {
            event_from(INDEX, self.index.pressed).map(|e| (e, self.index.changed_at_ms))
        } else {
            None
        }
    }
}
