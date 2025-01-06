//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

extern crate alloc;

use core::ptr::addr_of_mut;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    entry,
    gpio::{Level, Output},
};

use esp_backtrace as _;
use esp_println::println;

use fugit::ExtU64;

fn init_heap() {
    const HEAP_SIZE: usize = 64 * 1024;
    static mut HEAP: core::mem::MaybeUninit<[u8; HEAP_SIZE]> = core::mem::MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            addr_of_mut!(HEAP) as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

#[entry]
fn main() -> ! {
    init_heap();

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

    // alloc
    // see https://doc.rust-lang.org/stable/alloc/index.html
    {
        // use String
        use alloc::string::ToString;
        let s = "hello world!".to_string();
        println!("{:?}", s);

        // use Vec
        let array = alloc::vec![0u8; 12];
        println!("{:?}", array);

        // use BtreeMap
        let mut map = alloc::collections::BTreeMap::new();
        map.insert("hello", "world");
        println!("{:?}", map);

        // use Box
        let b = alloc::boxed::Box::new(12345678);
        println!("{:?}", b);
    }

    // json
    // see https://docs.rs/serde_json/1.0.116/serde_json/
    {
        use alloc::string::ToString;
        use alloc::vec;
        use serde_json::json;

        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });

        println!("first phone number: {}", john["phones"][0]);

        // Convert to a string of JSON and print it out
        println!("{}", john.to_string());

        let p: Person = serde_json::from_str(&john.to_string()).unwrap();

        // Do things just like with any other Rust data structure.
        println!("Please call {} at the number {}", p.name, p.phones[0]);
    }

    loop {
        println!("loop!");
        led.toggle();
        delay.delay_millis(500);
        led.toggle();
        // or using `fugit` duration
        delay.delay(2.secs());
    }
}

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
