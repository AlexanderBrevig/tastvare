use rp2040_hal::gpio::{
    bank0::{Gpio10, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9},
    Pin, PullUpInput,
};

use super::Six;
use crate::layout::StatefulGpio;

//TODO: consider moving the key map to a config file

#[cfg(feature = "left")]
type LR = Gpio5;
#[cfg(feature = "left")]
type LM = Gpio6;
#[cfg(feature = "left")]
type LI = Gpio7;

#[cfg(feature = "right")]
type RR = Gpio20;
#[cfg(feature = "right")]
type RM = Gpio19;
#[cfg(feature = "right")]
type RI = Gpio18;

#[cfg(feature = "left")]
pub(crate) fn left(
    ring: Pin<LR, PullUpInput>,
    middle: Pin<LM, PullUpInput>,
    index: Pin<LI, PullUpInput>,
) -> Twelve<LR, LM, LI> {
    Twelve::<LR, LM, LI> {
        ring: StatefulGpio::new(ring),
        middle: StatefulGpio::new(middle),
        index: StatefulGpio::new(index),
    }
}

#[cfg(feature = "right")]
pub(crate) fn right(
    ring: Pin<RR, PullUpInput>,
    middle: Pin<RM, PullUpInput>,
    index: Pin<RI, PullUpInput>,
) -> Twelve<LR, LM, LI> {
    Twelve::<LR, LM, LI> {
        ring: StatefulGpio::new(ring),
        middle: StatefulGpio::new(middle),
        index: StatefulGpio::new(index),
    }
}
