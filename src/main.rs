//! Docs

#![no_std]
#![no_main]

#[cfg(all(feature = "left", feature = "right"))]
compile_error!("Cannot be both left and right");

#[cfg(not(any(feature = "left", feature = "right")))]
compile_error!("Must be either left or right");

mod engine;
mod keymap;
mod layout;
mod light;
mod protocol;
mod serial;
mod usb;

use cortex_m::prelude::*;
use defmt::*;
use defmt_rtt as _;

use fugit::{ExtU32, RateExtU32};
use panic_probe as _;
use rp2040_hal::usb::UsbBus;
use serial::Serial;

use rp2040_hal::entry;
use rp2040_hal::{
    clocks::init_clocks_and_plls,
    gpio::{FunctionUart, Pins},
    pac,
    uart::{DataBits, StopBits, UartConfig, UartPeripheral},
    watchdog::Watchdog,
    Clock, Sio,
};
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig;
use usbd_human_interface_device::usb_class::UsbHidClassBuilder;

// USB

use crate::engine::Engine;
use crate::keymap::Keymap;
use crate::layout::Layout;
use crate::usb::Usb;

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000; // Typically found in BSP crates

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Testing USB API
    let mut pac = pac::Peripherals::take().unwrap();
    let mut _watchdog = Watchdog::new(pac.WATCHDOG);

    // Configure the addressable LED

    let uart_pins = (
        pins.gpio28.into_mode::<FunctionUart>(), //TX
        pins.gpio29.into_mode::<FunctionUart>(), //RX
    );

    // Top down design
    let mut engine = Engine::new();
    let keymap = keymap::tinykeys::Tinykeys {};

    #[cfg(feature = "left")]
    let mut layout = layout::sixbysix::tinykeys::left(
        pins.gpio5.into_pull_up_input(),
        pins.gpio6.into_pull_up_input(),
        pins.gpio7.into_pull_up_input(),
        pins.gpio8.into_pull_up_input(),
        pins.gpio9.into_pull_up_input(),
        pins.gpio10.into_pull_up_input(),
    );

    #[cfg(feature = "right")]
    let mut layout = layout::sixbysix::tinykeys::right(
        pins.gpio5.into_pull_up_input(),
        pins.gpio6.into_pull_up_input(),
        pins.gpio7.into_pull_up_input(),
        pins.gpio8.into_pull_up_input(),
        pins.gpio9.into_pull_up_input(),
        pins.gpio10.into_pull_up_input(),
    );

    //USB
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(NKROBootKeyboardConfig::default())
        .build(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("usbd-human-interface-device")
        .product("NKRO Keyboard")
        .serial_number("TEST")
        .build();
    let mut usb = Usb {
        usb_bus: &usb_bus,
        keyboard: &mut keyboard,
        usb_dev: &mut usb_dev,
    };

    let uart = UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(115_200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    let (mut rx, tx) = uart.split();
    let mut serial = Serial {
        tx: &tx,
        rx: &mut rx,
    };

    //TODO: place in a Tastvare::App
    while let Some(timed_event) = serial.get_event() {
        let event = engine.handle(timed_event);
        let report = keymap.generate_report(event);
        usb.write_report(report).ok();
    }

    while let Some(timed_event) = layout.get_event() {
        serial.send_event(timed_event.0);
        let event = engine.handle(timed_event);
        let report = keymap.generate_report(event);
        usb.write_report(report).ok();
    }

    // MAIN LOOP
    let timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS); //, &clocks);
    let mut tick_timer = timer.count_down();
    tick_timer.start(1.millis());
    let mut ms = 1000;
    loop {
        // tick once per ms/at 1kHz
        if tick_timer.wait().is_ok() {
            usb.tick().ok();
            //TODO: move to keyboard module and update module from here
            ms += 1;
            if ms >= 1000 {
                ms = 0;
            }
            //TODO: call into subsystems on time based interval from here
            // or use RTIC?
        }

        usb.poll();

        // let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        // info!("on!"); led_pin.set_high().unwrap(); delay.delay_ms(500);
    }
}
