//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    main,
    rmt::Rmt,
    time::{Duration, Rate},
};

use esp_backtrace as _;
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

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

    let rmt: Rmt<'_, esp_hal::Blocking> =
        Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to initialize RMT");

    use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
    // see https://docs.rs/smart-leds/latest/smart_leds/
    use smart_leds::{colors::*, SmartLedsWrite};

    let rmt_buffer = smart_led_buffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, rmt_buffer);

    let colors = [
        WHITE, SILVER, GRAY, BLACK, RED, MAROON, YELLOW, OLIVE, LIME, GREEN, AQUA, TEAL, BLUE,
        NAVY, FUCHSIA, PURPLE,
    ];

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let delay = Delay::new();

    loop {
        println!("loop!");

        for color in colors {
            let data = [color; 1];
            led.write(data).unwrap();

            delay.delay_millis(500);
        }

        // or using `fugit` duration
        delay.delay(Duration::from_secs(2));
    }
}
