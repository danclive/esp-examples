//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

extern crate alloc;

use core::ptr::addr_of_mut;

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    entry,
    gpio::{Level, Output},
};
use esp_println::println;

use fugit::ExtU64;

fn init_heap() {
    const HEAP_SIZE: usize = 128 * 1024;
    static mut HEAP: core::mem::MaybeUninit<[u8; HEAP_SIZE]> = core::mem::MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            addr_of_mut!(HEAP) as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

use core::cell::RefCell;
use critical_section::Mutex;

static LED: Mutex<RefCell<Option<Output<'static>>>> = Mutex::new(RefCell::new(None));

fn toggle_led() {
    critical_section::with(|cs| {
        if let Some(led) = LED.borrow_ref_mut(cs).as_mut() {
            led.toggle();
        }
    })
}

// Normal function that returns a standard type
// Remember to use 'ImmutableString' and not 'String'
fn add_len(x: i32, s: rhai::ImmutableString) -> i32 {
    x + s.len() as i32
}
// Alternatively, '&str' maps directly to 'ImmutableString'
fn add_len_count(x: i32, s: &str, c: i32) -> i32 {
    x + s.len() as i32 * c
}
// Function that returns a 'Dynamic' value
fn get_any_value() -> rhai::Dynamic {
    42_i32.into() // standard types can use '.into()'
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

    led.set_high();

    critical_section::with(|cs| {
        LED.borrow_ref_mut(cs).replace(led);
    });

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

    // rhai
    // see https://rhai.rs/
    let mut engine = rhai::Engine::new();

    // sprint and debug
    // see https://rhai.rs/book/language/print-debug.html
    {
        engine.on_print(|s| println!("{s}"));
        engine.on_debug(|s, src, pos| match (src, pos) {
            (Some(source), rhai::Position::NONE) => println!("{source} | {s}"),
            (Some(source), pos) => println!("{source} @ {pos:?} | {s}"),
            (None, rhai::Position::NONE) => println!("{s}"),
            (None, pos) => println!("{pos:?} | {s}"),
        });
    }

    // Register rust function
    // see https://rhai.rs/book/rust/functions.html
    {
        engine.register_fn("toggle", toggle_led);

        engine
            .register_fn("add", add_len)
            .register_fn("add", add_len_count)
            .register_fn("add", get_any_value)
            .register_fn("inc", |x: i32| {
                // closure is also OK!
                x + 1
            })
            .register_fn("log", |label: &str, x: i32| {
                println!("{label} = {x}");
            });

        let result = engine.eval::<i32>(r#"add(40, "xx")"#).unwrap();

        println!("Answer: {result}"); // prints 42

        let result = engine.eval::<i32>(r#"add(40, "x", 2)"#).unwrap();

        println!("Answer: {result}"); // prints 42

        let result = engine.eval::<i32>("add()").unwrap();

        println!("Answer: {result}"); // prints 42

        let result = engine.eval::<i32>("inc(41)").unwrap();

        println!("Answer: {result}"); // prints 42

        engine.run(r#"log("value", 42)"#).unwrap(); // prints "value = 42"
    }

    let ast = engine
        .compile(
            r#"
            fn get_message() {
                "Hello!"              // greeting message
            }

            fn say_hello() {
                print(get_message());   // prints message
            }

            say_hello();
            toggle();
        "#,
        )
        .unwrap();

    loop {
        println!("loop!");
        engine.run_ast(&ast).unwrap();
        delay.delay_millis(500);
        engine.run_ast(&ast).unwrap();
        // or using `fugit` duration
        delay.delay(2.secs());
    }
}
