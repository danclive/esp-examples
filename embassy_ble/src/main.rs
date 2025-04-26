//! Connect to wifi and use TCP
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

extern crate alloc;

use core::cell::RefCell;
use core::ptr::addr_of_mut;

use esp_hal::{
    clock::CpuClock,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    rng::Rng,
    time,
    timer::timg::TimerGroup,
};

use esp_backtrace as _;
use esp_println::println;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_wifi::{EspWifiController, ble::controller::BleConnector, init};

use bleps::{
    ad_structure::{
        AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE, create_advertising_data,
    },
    async_attribute_server::AttributeServer,
    asynch::Ble,
    attribute_server::NotificationData,
    gatt,
};

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

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

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    init_heap();

    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    println!("Init!");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // init wifi
    let timer1 = TimerGroup::new(peripherals.TIMG1);
    let rng = Rng::new(peripherals.RNG);

    let esp_wifi_ctrl = &*mk_static!(
        EspWifiController<'static>,
        init(timer1.timer0, rng, peripherals.RADIO_CLK).unwrap()
    );
    // let init = &*singleton!(:EspWifiController<'static> = init(timer1.timer0, rng, peripherals.RADIO_CLK).unwrap()).unwrap();

    let mut bluetooth = peripherals.BT;

    let connector = BleConnector::new(esp_wifi_ctrl, &mut bluetooth);

    let now = || time::Instant::now().duration_since_epoch().as_millis();
    let mut ble = Ble::new(connector, now);
    println!("Connector created");

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
    let led = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());

    let button = Input::new(
        peripherals.GPIO9,
        InputConfig::default().with_pull(Pull::Down),
    );

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    println!("embassy init!");

    spawner.spawn(run()).ok();
    spawner.spawn(toggle(led)).ok();

    let pin_ref = RefCell::new(button);
    let pin_ref = &pin_ref;

    loop {
        println!("{:?}", ble.init().await);
        println!("{:?}", ble.cmd_set_le_advertising_parameters().await);
        println!(
            "{:?}",
            ble.cmd_set_le_advertising_data(
                create_advertising_data(&[
                    AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                    AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1809)]),
                    AdStructure::CompleteLocalName(esp_hal::chip!()),
                    AdStructure::ServiceData16 {
                        uuid: 0x1809,
                        data: b"ESP-BLE",
                    },
                ])
                .unwrap()
            )
            .await
        );
        println!("{:?}", ble.cmd_set_le_advertise_enable(true).await);

        println!("started advertising");

        let mut rf = |_offset: usize, data: &mut [u8]| {
            data[..20].copy_from_slice(&b"Hello Bare-Metal BLE"[..]);
            17
        };
        let mut wf = |offset: usize, data: &[u8]| {
            println!("RECEIVED: {} {:?}", offset, data);
        };

        let mut wf2 = |offset: usize, data: &[u8]| {
            println!("RECEIVED: {} {:?}", offset, data);
        };

        let mut rf3 = |_offset: usize, data: &mut [u8]| {
            data[..5].copy_from_slice(&b"Hola!"[..]);
            5
        };
        let mut wf3 = |offset: usize, data: &[u8]| {
            println!("RECEIVED: Offset {}, data {:?}", offset, data);
        };

        gatt!([service {
            uuid: "937312e0-2354-11eb-9f10-fbc30a62cf38",
            characteristics: [
                characteristic {
                    uuid: "937312e0-2354-11eb-9f10-fbc30a62cf38",
                    read: rf,
                    write: wf,
                },
                characteristic {
                    uuid: "957312e0-2354-11eb-9f10-fbc30a62cf38",
                    write: wf2,
                },
                characteristic {
                    name: "my_characteristic",
                    uuid: "987312e0-2354-11eb-9f10-fbc30a62cf38",
                    notify: true,
                    read: rf3,
                    write: wf3,
                },
            ],
        },]);

        let mut rng = bleps::no_rng::NoRng;
        let mut srv = AttributeServer::new(&mut ble, &mut gatt_attributes, &mut rng);

        let counter = RefCell::new(0u8);
        let counter = &counter;

        let mut notifier = || {
            // TODO how to check if notifications are enabled for the characteristic?
            // maybe pass something into the closure which just can query the characteristic
            // value probably passing in the attribute server won't work?

            async {
                let mut button = pin_ref.borrow_mut();
                button.wait_for_rising_edge().await;
                println!("button pressed");

                println!("sending notification");
                let mut data = [0u8; 13];
                data.copy_from_slice(b"Notification0");
                {
                    let mut counter = counter.borrow_mut();
                    data[data.len() - 1] += *counter;
                    *counter = (*counter + 1) % 10;
                }
                NotificationData::new(my_characteristic_handle, &data)
            }
        };

        srv.run(&mut notifier).await.unwrap();
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
async fn toggle(mut led: Output<'static>) {
    loop {
        println!("toggle loop!");
        led.toggle();
        Timer::after_secs(1).await;
        led.toggle();
        Timer::after_secs(2).await;
    }
}
