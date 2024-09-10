# Embassy WIFI

This example shows how to turn on basic WIFI support in the [Embassy](https://github.com/embassy-rs/embassy) framework.

> 这个例子演示了如何在 [Embassy](https://github.com/embassy-rs/embassy) 框架下开启最基本的 WIFI 支持。

This example is modified from [embassy_blinky](../embassy_blinky). If you are just starting out, I suggest you read [embassy_blinky](../embassy_blinky) before continuing down the page

> 这个例子是在 [embassy_blinky](../embassy_blinky) 的基础上修改而来的。如果你是刚开始，我建议你先阅读 [embassy_blinky](../embassy_blinky) 的内容，再继续往下阅读。

To turn on basic WIFI support, we just need to add the following to `Cargo.toml`:

> 开启 WIFI 基本的支持，我们只需要在 `Cargo.toml` 中添加以下内容:

```toml
[features]
default = [
    ...
    "wifi",
]

log = [
    ...
    "esp-wifi?/log",
]

defmt = [
    ...
    "esp-wifi?/defmt",
    "embassy-net?/defmt"
]

esp32c3 = [..., "esp-wifi?/esp32c3"]
esp32c6 = [..., "esp-wifi?/esp32c6"]

[dependencies]
esp-wifi  = { version = "0.9", default-features = false, features = [
    "wifi", "ipv4", "tcp", "udp", "icmp", "igmp", "dns", "dhcpv4"], optional = true }
embassy-net = { version = "0.4", features = ["tcp", "udp", "dhcpv4", "medium-ethernet"], optional = true }
```

And add to the `.cargo/config.toml` file:

> 并在 `.cargo/config.toml` 文件中添加：

```toml
rustflags = [
  ...
  # with wifi
  "-C", "link-arg=-Trom_functions.x",
]
```

One thing to note is that after turning on WIFI, you have to let the chip run at the maximum clock speed, otherwise you will be told that the clock is misconfigured when initializing WIFI. I'm not sure if this is a bug or not.

> 要注意的是，开启 WIFI 后，要让芯片运行在最大时钟速度，否则初始化 WIFI 时会被告知时钟配置错误。我不知道这是否是 BUG。

```rust
let clocks = ClockControl::max(system.clock_control).freeze();
```
