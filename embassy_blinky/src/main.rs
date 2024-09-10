//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

use esp_hal::{
    clock::ClockControl,
    gpio::{AnyOutput, Io, Level},
    peripherals::Peripherals,
    system::SystemControl,
};

use esp_hal::macros::main;

use esp_backtrace as _;
use esp_println::println;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[main]
async fn main(spawner: Spawner) {
    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    println!("Init!");

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
    let mut led = AnyOutput::new(io.pins.gpio8, Level::Low);

    led.set_high();

    use esp_hal::timer::systimer::{SystemTimer, Target};
    let systimer = SystemTimer::new(peripherals.SYSTIMER).split::<Target>();
    esp_hal_embassy::init(&clocks, systimer.alarm0);

    println!("embassy init!");

    spawner.spawn(run()).ok();
    spawner.spawn(toggle(led)).ok();

    loop {
        println!("main loop!");
        Timer::after(Duration::from_millis(5_000)).await;
        esp_hal::riscv::asm::wfi();
    }
}

#[embassy_executor::task]
async fn run() {
    loop {
        println!("task loop!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[embassy_executor::task]
async fn toggle(mut led: AnyOutput<'static>) {
    loop {
        println!("toggle loop!");
        led.toggle();
        Timer::after_secs(1).await;
        led.toggle();
        Timer::after_secs(2).await;
    }
}
