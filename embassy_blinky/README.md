# Embassy Blinky

[Embassy](https://github.com/embassy-rs/embassy) is an embedded runtime framework that supports `async/await` for simple and efficient multitasking in embedded systems.

> [Embassy](https://github.com/embassy-rs/embassy) 是一个支持 `async/await` 的嵌入式运行框架, 能够在嵌入式系统中实现简单高效的多任务处理。

This example is modified from [blinky](../blinky). If you are just starting out, I suggest you read [blinky](../blinky) before continuing on.

> 这个例子是在 [blinky](../blinky) 的基础上修改而来的。如果你是刚开始，我建议你先阅读 [blinky](../blinky) 的内容，再继续往下阅读。

To turn on basic embassy support, we just need to add the following to `Cargo.toml`.

开启 embassy 基本的支持，我们只需要在 `Cargo.toml` 中添加以下内容:

```toml
[features]
default = [
    ...
    "embassy"
]

log = [
    ...
    "embassy-executor?/log",
]

defmt = [
    ...
    "esp-hal-embassy?/defmt",
    "embassy-executor?/defmt",
]

embassy = [
    "esp-hal-embassy",
    "embassy-executor",
    "embassy-time",
    "embassy-executor/task-arena-size-20480"
]

esp32c3 = [..., "esp-hal-embassy?/esp32c3"]
esp32c6 = [..., "esp-hal-embassy?/esp32c6"]

[dependencies]
esp-hal-embassy = { version = "0.6", optional = true  }
embassy-executor = { version = "0.7", package = "embassy-executor", features = ["arch-riscv32"], optional = true }
embassy-time = { version = "0.4", features = ["generic-queue-8"], optional = true }
```

After initializing embassy, you can use multitasking, which is very different from traditional embedded development.

> 初始化 embassy 之后，你就可以使用多任务, 这与传统嵌入式开发是截然不同的。

```rust
#[embassy_executor::main(entry = "esp_hal::entry")]
async fn main(spawner: Spawner) {
    ...
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    println!("embassy init!");

    spawner.spawn(run()).ok();
    spawner.spawn(toggle(led)).ok();

    loop {
        println!("main loop!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
    ...
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
```
