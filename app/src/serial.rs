use cortex_m::prelude::_embedded_hal_serial_Read;
use rp2040_hal::uart::{Reader, UartDevice, ValidUartPinout, Writer};

use tast::{
    protocol::{Event, TimedEvent},
    serial::EventSource,
};

pub struct Serial<'a, U, TX, RX>
where
    U: UartDevice,
    TX: ValidUartPinout<U>,
    RX: ValidUartPinout<U>,
{
    pub tx: &'a Writer<U, RX>,
    pub rx: &'a mut Reader<U, TX>,
}

impl<U, TX, RX> EventSource for Serial<'_, U, TX, RX>
where
    U: UartDevice,
    TX: ValidUartPinout<U>,
    RX: ValidUartPinout<U>,
{
    #[cfg(feature = "left")]
    fn get_event(&mut self) -> Option<TimedEvent> {
        let word = self.rx.read().unwrap_or_default();
        if word != 0 {
            //TODO: get timestamp
            Some((Event::from_bits(word)?, 10u32))
        } else {
            None
        }
    }
    #[cfg(feature = "right")]
    fn get_event(&self) -> Option<TimedEvent> {
        None
    }
    #[cfg(feature = "left")]
    fn send_event(&self, event: Event) {
        self.tx.write_full_blocking(&[event.bits()]);
    }
    #[cfg(feature = "right")]
    fn send_event(&self, event: Event) {}
}
