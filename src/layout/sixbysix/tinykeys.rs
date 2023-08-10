use rp2040_hal::gpio::{
    bank0::{Gpio10, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9},
    Pin, PullUpInput,
};

use super::SixBySix;
use crate::layout::StatefulGpio;

//TODO: consider moving the key map to a config file

#[cfg(feature = "left")]
type LP = Gpio5;
#[cfg(feature = "left")]
type LR = Gpio6;
#[cfg(feature = "left")]
type LM = Gpio7;
#[cfg(feature = "left")]
type LI = Gpio8;
#[cfg(feature = "left")]
type LT1 = Gpio9;
#[cfg(feature = "left")]
type LT2 = Gpio10;

#[cfg(feature = "right")]
type RP = Gpio5;
#[cfg(feature = "right")]
type RR = Gpio6;
#[cfg(feature = "right")]
type RM = Gpio7;
#[cfg(feature = "right")]
type RI = Gpio8;
#[cfg(feature = "right")]
type RT1 = Gpio9;
#[cfg(feature = "right")]
type RT2 = Gpio10;

#[cfg(feature = "left")]
pub(crate) fn left(
    pinky: Pin<LP, PullUpInput>,
    ring: Pin<LR, PullUpInput>,
    middle: Pin<LM, PullUpInput>,
    index: Pin<LI, PullUpInput>,
    thumb1: Pin<LT1, PullUpInput>,
    thumb2: Pin<LT2, PullUpInput>,
) -> SixBySix<LP, LR, LM, LI, LT1, LT2> {
    SixBySix::<LP, LR, LM, LI, LT1, LT2> {
        pinky: StatefulGpio::new(pinky),
        ring: StatefulGpio::new(ring),
        middle: StatefulGpio::new(middle),
        index: StatefulGpio::new(index),
        thumb1: StatefulGpio::new(thumb1),
        thumb2: StatefulGpio::new(thumb2),
    }
}

#[cfg(feature = "right")]
pub(crate) fn right(
    pinky: Pin<RP, PullUpInput>,
    ring: Pin<RR, PullUpInput>,
    middle: Pin<RM, PullUpInput>,
    index: Pin<RI, PullUpInput>,
    thumb1: Pin<RT1, PullUpInput>,
    thumb2: Pin<RT2, PullUpInput>,
) -> SixBySix<RP, RR, RM, RI, RT1, RT2> {
    SixBySix::<RP, RR, RM, RI, RT1, RT2> {
        pinky: StatefulGpio::new(pinky),
        ring: StatefulGpio::new(ring),
        middle: StatefulGpio::new(middle),
        index: StatefulGpio::new(index),
        thumb1: StatefulGpio::new(thumb1),
        thumb2: StatefulGpio::new(thumb2),
    }
}
