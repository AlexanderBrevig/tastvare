use defmt_rtt as _;
use frunk::{HCons, HNil};
use rp2040_hal::usb::UsbBus;
use tast::report::UsbReporter;
use usb_device::{class_prelude::UsbBusAllocator, prelude::UsbDevice};
use usbd_human_interface_device::{
    device::keyboard::NKROBootKeyboard, page::Keyboard, usb_class::UsbHidClass, UsbHidError,
};

pub struct Usb<'a> {
    pub keyboard: &'a mut UsbHidClass<'a, UsbBus, HCons<NKROBootKeyboard<'a, UsbBus>, HNil>>,
    pub usb_dev: &'a mut UsbDevice<'a, UsbBus>,
    pub usb_bus: &'a UsbBusAllocator<UsbBus>,
}

impl<'a> UsbReporter for Usb<'a> {
    fn poll(&mut self) {
        if self.usb_dev.poll(&mut [self.keyboard]) {
            if let Ok(_l) = self.keyboard.device().read_report() {
                //TODO: handle this
                // update_leds(l);
            }
        }
    }
    fn write_report<K: IntoIterator<Item = Keyboard>>(
        &mut self,
        keys: Option<K>,
    ) -> Result<(), UsbHidError> {
        let keys = keys.ok_or(UsbHidError::SerializationError)?;
        self.keyboard.device().write_report(keys)
    }
    fn tick(&mut self) -> Result<(), UsbHidError> {
        self.keyboard.tick()
    }
}
