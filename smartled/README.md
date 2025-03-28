# SMART LED

This example shows how to use the RMT to light an RGB LED, such as the WS2812, assuming you already know about RGB LEDs such as the WS2812, which is not covered in this example.

> 这个例子展示了如何利用 RMT 点亮 RGB LED，例如 WS2812。假设你已经了解 WS2812 这类 RGB LED 相关知识，本示例中不做过多介绍。

This example is modified from [blinky](../blinky). If you don't understand something, you can read the contents of [blinky](../blinky) before continuing on.

> 这个例子是在 [blinky](../blinky) 的基础上修改而来的。如果你有不懂的地方，你可以先阅读 [blinky](../blinky) 的内容，再继续往下阅读。

To light up the RGB LEDs, we just need to add the following to the `Cargo.toml` file:

> 要点亮 RGB LED，我们只需要在 `Cargo.toml` 文件中添加以下内容：

```toml
[features]
default = [
    ...
    "smartled"
]

defmt = [
    ...
    "esp-hal-smartled?/defmt"
]

esp32c3 = [..., "esp-hal-smartled?/esp32c3"]
esp32c6 = [..., "esp-hal-smartled?/esp32c6"]

smartled = [
    "esp-hal-smartled",
    "smart-leds",
]

[dependencies]
...
esp-hal-smartled = { version = "0.14", optional = true}
smart-leds = { version = "0.4", optional = true}
```

[esp-hal-smartled](https://crates.io/crates/esp-hal-smartled) has already done the necessary work to light the RGB LEDs for us, [smart-leds](https://docs.rs/smart-leds/latest/ smart_leds/) contains some of the components needed.

> [esp-hal-smartled](https://crates.io/crates/esp-hal-smartled) 已经为我们做好了点亮 RGB LED 的必要工作，[smart-leds](https://docs.rs/smart-leds/latest/smart_leds/) 包含了一些需要用到的组件。

Then add the following code to the `main.rs` file:

> 然后在 `main.rs` 文件中添加以下代码：

```rust
let rmt = Rmt::new(peripherals.RMT, 80.MHz()).unwrap();

use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
// see https://docs.rs/smart-leds/latest/smart_leds/
use smart_leds::{colors::*, SmartLedsWrite};

let rmt_buffer = smartLedBuffer!(1); // Number of LEDs is 1
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
        let data = [color; 1]; // Number of LEDs is 1
        led.write(data).unwrap();

        delay.delay_millis(500);
    }

    // or using `fugit` duration
    delay.delay(2.secs());
}
```

The [ESP32-C6 development board](https://docs.espressif.com/projects/espressif-esp-dev-kits/zh_CN/latest/esp32c6/esp32-c6-devkitc-1/user_guide. html#id9) integrates one RGB LED connected to the GPIO8 pin. If you want to connect more RGB LEDs, remember to change the number of LEDs in `smartLedBuffer!(1)` and `let data = [color; 1];`.

> 我使用的 [ESP32-C6 开发板](https://docs.espressif.com/projects/espressif-esp-dev-kits/zh_CN/latest/esp32c6/esp32-c6-devkitc-1/user_guide.html#id9) 集成了一颗 RGB LED，连接在 GPIO8 引脚上。如果你要连接更多 RGB LED，记得修改 `smartLedBuffer!(1)` 和 `let data = [color; 1];` 中的 LED 数量。
