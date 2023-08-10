/*
use rp2040_hal::pac;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;
pub fn placehold() {
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        pins.gpio24.into_mode(),
        //pins.gpio27.into_mode(), // tinykeys
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    ws.write(brightness(once(wheel(n)), 32)).unwrap();
    n = n.wrapping_add(1);
}
*/
