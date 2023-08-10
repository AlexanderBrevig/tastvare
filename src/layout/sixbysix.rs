use rp2040_hal::gpio::{PinId, PullUpInput};

use super::{Layout, StatefulGpio};
use crate::protocol::{Event, TimedEvent};

pub mod tinykeys;

const THUMB2: u8 = Event::ID5.bits();
const THUMB1: u8 = Event::ID4.bits();
const INDEX: u8 = Event::ID3.bits();
const MIDDLE: u8 = Event::ID2.bits();
const RING: u8 = Event::ID1.bits();
const PINKY: u8 = Event::ID0.bits();

pub struct SixBySix<P, R, M, I, T1, T2>
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

fn event_from(mask: u8, pressed: bool) -> Option<Event> {
    let mut event = Event::from_bits(mask)?;
    if pressed {
        event |= Event::PRESSED;
    }
    Some(event)
}

impl<P, R, M, I, T1, T2> Layout for SixBySix<P, R, M, I, T1, T2>
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
