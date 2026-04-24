//! Connect to wifi and use TCP
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO8)

//% CHIPS: esp32c3 esp32c6

#![no_std]
#![no_main]

extern crate alloc;

use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    ram,
    rng::Rng,
    timer::timg::TimerGroup,
};

use esp_alloc as _;
use esp_backtrace as _;
use esp_println::println;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_radio::wifi::{self, ControllerConfig, Interface, WifiController, sta::StationConfig};

use embassy_net::{self, Ipv4Address, Runner, Stack, StackResources, tcp::TcpSocket};

const SSID: &str = "HOME";
const PASSWORD: &str = "12345678";

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write($val);
        x
    }};
}

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    println!("Init!");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 36 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    println!("esp_rtos start!");

    let station_config = wifi::Config::Station(
        StationConfig::default()
            .with_ssid(SSID)
            .with_password(PASSWORD.into()),
    );

    println!("Starting wifi");
    let (controller, interfaces) = esp_radio::wifi::new(
        peripherals.WIFI,
        ControllerConfig::default().with_initial_config(station_config),
    )
    .unwrap();
    println!("Wifi configured and started!");

    let wifi_interface = interfaces.station;

    let config = embassy_net::Config::dhcpv4(Default::default());

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    // Init network stack
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

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

    spawner.spawn(run().unwrap());
    spawner.spawn(toggle(led).unwrap());

    spawner.spawn(connection(controller).unwrap());
    spawner.spawn(net_task(runner).unwrap());
    spawner.spawn(tcp(stack).unwrap());

    loop {
        println!("main loop!");
        Timer::after(Duration::from_millis(5_000)).await;
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

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");

    loop {
        println!("About to connect...");

        if controller.is_connected() {
            println!("Already connected!");
            Timer::after(Duration::from_millis(5000)).await;
            continue;
        }

        match controller.connect_async().await {
            Ok(_) => println!("Wifi connected!"),
            Err(e) => {
                println!("Failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await
}

#[embassy_executor::task]
async fn tcp(stack: Stack<'static>) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    stack.wait_config_up().await;

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    loop {
        Timer::after(Duration::from_millis(1_000)).await;

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        socket.set_timeout(Some(Duration::from_secs(10)));

        let remote_endpoint = (Ipv4Address::new(3, 234, 199, 80), 80);
        println!("connecting...");
        let r = socket.connect(remote_endpoint).await;
        if let Err(e) = r {
            println!("connect error: {:?}", e);
            continue;
        }
        println!("connected!");
        let mut buf = [0; 1024];
        loop {
            let r = write_all(
                &mut socket,
                b"GET /get HTTP/1.0\r\nHost: httpbin.org\r\n\r\n",
            )
            .await;
            if let Err(e) = r {
                println!("write error: {:?}", e);
                break;
            }

            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    println!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    println!("read error: {:?}", e);
                    break;
                }
            };
            println!("{}", core::str::from_utf8(&buf[..n]).unwrap());
        }
        Timer::after(Duration::from_millis(3000)).await;
    }
}

async fn write_all(socket: &mut TcpSocket<'_>, buf: &[u8]) -> Result<(), embassy_net::tcp::Error> {
    let mut buf = buf;
    while !buf.is_empty() {
        match socket.write(buf).await {
            Ok(0) => panic!("write() returned Ok(0)"),
            Ok(n) => buf = &buf[n..],
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
