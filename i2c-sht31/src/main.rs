//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    entry,
    gpio::{Level, Output},
    i2c::master::{Config, I2c},
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

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        // Configure the CPU to run at the maximum frequency.
        config.cpu_clock = CpuClock::max();
        config
    });

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
    let mut led = Output::new(peripherals.GPIO8, Level::High);

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let delay = Delay::new();

    let mut i2c = I2c::new(
        peripherals.I2C0,
        Config {
            frequency: 100.kHz(),
            timeout: None,
        },
    )
    .with_sda(peripherals.GPIO4)
    .with_scl(peripherals.GPIO5);

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
