use usbd_human_interface_device::{page::Keyboard, UsbHidError};

pub trait UsbReporter {
    fn tick(&mut self) -> Result<(), UsbHidError>;
    fn poll(&mut self);
    fn write_report<K: IntoIterator<Item = Keyboard>>(
        &mut self,
        keys: Option<K>,
    ) -> Result<(), UsbHidError>;
}
