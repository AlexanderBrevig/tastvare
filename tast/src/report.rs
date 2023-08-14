use usbd_human_interface_device::{page::Keyboard, UsbHidError};

pub trait UsbReporter {
    fn tick(&mut self) -> Result<(), UsbHidError>;
    fn poll(&mut self);
    fn write_report<K: IntoIterator<Item = Keyboard>>(
        &mut self,
        keys: Option<K>,
    ) -> Result<(), UsbHidError>;
}

#[cfg(test)]
pub mod tests {
    use super::UsbReporter;

    pub struct TestUsbReporter {}
    impl UsbReporter for TestUsbReporter {
        fn tick(&mut self) -> Result<(), usbd_human_interface_device::UsbHidError> {
            Ok(())
        }

        fn poll(&mut self) {}

        fn write_report<K: IntoIterator<Item = usbd_human_interface_device::page::Keyboard>>(
            &mut self,
            keys: Option<K>,
        ) -> Result<(), usbd_human_interface_device::UsbHidError> {
            Ok(())
        }
    }
}
