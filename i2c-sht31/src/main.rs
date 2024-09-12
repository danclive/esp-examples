//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    entry,
    gpio::{Io, Level, Output},
    i2c::I2C,
    peripherals::Peripherals,
    system::SystemControl,
};

use esp_backtrace as _;
use esp_println::println;

use fugit::{ExtU64, RateExtU32};

use sht3x::{Address, Sht3x};

pub mod sht3x;

#[entry]
fn main() -> ! {
    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
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
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio8, Level::Low);

    led.set_high();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let delay = Delay::new(&clocks);

    let mut i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio4,
        io.pins.gpio5,
        100.kHz(),
        &clocks,
    );

    let mut sht3x = Sht3x::new(&mut i2c, Address::Low, delay);

    loop {
        println!("loop!");
        led.toggle();
        delay.delay_millis(500);
        led.toggle();
        // or using `fugit` duration
        delay.delay(2.secs());

        let res = sht3x.measure(sht3x::ClockStretch::Enabled, sht3x::Repeatability::High);
        println!("sht3x.measure: {:?}", res);
    }
}
