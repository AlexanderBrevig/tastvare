use rp2040_hal::gpio::{
    bank0::{Gpio18, Gpio19, Gpio20, Gpio5, Gpio6, Gpio7},
    Pin, PullUpInput,
};

use super::Six;
use crate::layout::StatefulGpio;

//TODO: consider moving the key map to a config file

#[allow(unused)]
type LS = Gpio5;
#[allow(unused)]
type LL = Gpio6;
#[allow(unused)]
type LR = Gpio7;
#[allow(unused)]
type RL = Gpio18;
#[allow(unused)]
type RR = Gpio19;
#[allow(unused)]
type RS = Gpio20;

#[allow(unused)]
pub(crate) fn microkeys(
    lshift: Pin<LS, PullUpInput>,
    lleft: Pin<LL, PullUpInput>,
    lright: Pin<LR, PullUpInput>,
    rleft: Pin<RL, PullUpInput>,
    rright: Pin<RR, PullUpInput>,
    rshift: Pin<RS, PullUpInput>,
) -> Six<LS, LL, LR, RL, RR, RS> {
    Six::<LS, LL, LR, RL, RR, RS> {
        lshift: StatefulGpio::new(lshift),
        lleft: StatefulGpio::new(lleft),
        lright: StatefulGpio::new(lright),
        rleft: StatefulGpio::new(rleft),
        rright: StatefulGpio::new(rright),
        rshift: StatefulGpio::new(rshift),
    }
}
