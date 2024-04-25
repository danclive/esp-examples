//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

use esp_hal::{
    clock::ClockControl, delay::Delay, entry, gpio::IO, peripherals::Peripherals, rmt::Rmt,
    system::SystemExt,
};

use esp_backtrace as _;
use esp_println::println;

use fugit::{ExtU64, RateExtU32};

#[entry]
fn main() -> ! {
    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // use esp_println
    println!("hello world!");

    // use log
    #[cfg(feature = "log")]
    {
        log::error!("this is error message");
        log::warn!("this is warn message");
        log::info!("this is info message");
        log::debug!("this is debug message");
        log::trace!("this is trace message");
    }

    // Set GPIO0 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let rmt = Rmt::new(peripherals.RMT, 80.MHz(), &clocks, None).unwrap();

    use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
    // see https://docs.rs/smart-leds/latest/smart_leds/
    use smart_leds::{colors::*, SmartLedsWrite};

    let rmt_buffer = smartLedBuffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, io.pins.gpio8, rmt_buffer, &clocks);

    let colors = [
        WHITE, SILVER, GRAY, BLACK, RED, MAROON, YELLOW, OLIVE, LIME, GREEN, AQUA, TEAL, BLUE,
        NAVY, FUCHSIA, PURPLE,
    ];

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let delay = Delay::new(&clocks);

    loop {
        println!("loop!");

        for color in colors {
            let data = [color; 1];
            led.write(data).unwrap();

            delay.delay_millis(500);
        }

        // or using `fugit` duration
        delay.delay(2.secs());
    }
}
