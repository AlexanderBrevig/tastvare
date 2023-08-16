use rp2040_hal::gpio::{PinId, PullUpInput};

use tast::{
    layout::{
        twelve::event_from,
        twelve::{INDEX, MIDDLE, PINKY, RING, THUMB1, THUMB2},
        Layout,
    },
    protocol::TimedEvent,
};

use super::StatefulGpio;
pub mod tinykeys;

pub struct Twelve<P, R, M, I, T1, T2>
where
    P: PinId,
    R: PinId,
    M: PinId,
    I: PinId,
    T1: PinId,
    T2: PinId,
{
    pinky: StatefulGpio<P, PullUpInput>,
    ring: StatefulGpio<R, PullUpInput>,
    middle: StatefulGpio<M, PullUpInput>,
    index: StatefulGpio<I, PullUpInput>,
    thumb1: StatefulGpio<T1, PullUpInput>,
    thumb2: StatefulGpio<T2, PullUpInput>,
}

impl<P, R, M, I, T1, T2> Layout for Twelve<P, R, M, I, T1, T2>
where
    P: PinId,
    R: PinId,
    M: PinId,
    I: PinId,
    T1: PinId,
    T2: PinId,
{
    fn get_event(&mut self) -> Option<TimedEvent> {
        if let Ok(true) = self.pinky.did_change() {
            event_from(PINKY, self.pinky.pressed).map(|e| (e, self.pinky.changed_at_ms))
        } else if let Ok(true) = self.ring.did_change() {
            event_from(RING, self.ring.pressed).map(|e| (e, self.ring.changed_at_ms))
        } else if let Ok(true) = self.middle.did_change() {
            event_from(MIDDLE, self.middle.pressed).map(|e| (e, self.middle.changed_at_ms))
        } else if let Ok(true) = self.index.did_change() {
            event_from(INDEX, self.index.pressed).map(|e| (e, self.index.changed_at_ms))
        } else if let Ok(true) = self.thumb1.did_change() {
            event_from(THUMB1, self.thumb1.pressed).map(|e| (e, self.thumb1.changed_at_ms))
        } else if let Ok(true) = self.thumb2.did_change() {
            event_from(THUMB2, self.thumb2.pressed).map(|e| (e, self.thumb2.changed_at_ms))
        } else {
            None
        }
    }
}
