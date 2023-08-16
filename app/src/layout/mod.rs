use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;
use rp2040_hal::gpio::{Pin, PinId, PinMode, PullUpInput, ValidPinMode};
use tast::protocol::EventTime;

pub mod thirtyfour;
pub mod twelve;

pub struct StatefulGpio<I, M>
where
    I: PinId,
    M: PinMode + ValidPinMode<I>,
{
    pin: Pin<I, M>,
    pressed: bool,
    changed_at_ms: EventTime,
}

impl<I> StatefulGpio<I, PullUpInput>
where
    I: PinId,
{
    pub fn new(pin: Pin<I, PullUpInput>) -> StatefulGpio<I, PullUpInput> {
        StatefulGpio {
            pin,
            pressed: false,
            changed_at_ms: 0,
        }
    }

    pub fn did_change(&mut self) -> Result<bool, Infallible> {
        let is_pressed = self.pin.is_low()?;
        if self.pressed != is_pressed {
            self.pressed = is_pressed;
            if self.pressed {
                self.changed_at_ms = 10; //TODO: actual time
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
