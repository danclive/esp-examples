# Blinky

This is the simplest example that demonstrates how to light the LEDs and how to output the logs. The default chip model is esp32c6, you need to modify the `Cargo.toml` file and the `.cargo/config.toml` file.

> 这是一个最简单的示例，演示了如何点亮LED和如何输出日志。默认的芯片型号是 esp32c6, 你需要修改 `Cargo.toml` 文件和 `.cargo/config.toml` 文件。

```toml
[features]
default = [
    "esp32c6",
    ...
]
```

```toml
[build]
# esp32c3
# target = "riscv32imc-unknown-none-elf"
# esp32c6
target = "riscv32imac-unknown-none-elf"
```

For models with built-in JTAG, you can choose to output logs via JTAG or UART, otherwise you can only output via UART. if you output logs from JTAG, you can use the UART for other purposes. In this example the default is to output logs via JTAG, but I've added options that you can change in the `Cargo.toml` file.

> 对于内置 JTAG 的型号，可以选择通过 JTAG 或者 UART 输出日志，否则只能通过 UART。如果从 JTAG 输出日志，你可以将 UART 用于其他用途。这个例子中默认通过 JTAG 输出日志，不过我已经添加好了选项，你可以在 `Cargo.toml` 文件中修改。

```toml
[features]
default = [
    ...
    "jtag",
    ...
]

jtag = ["esp-println/jtag-serial"]
uart = ["esp-println/uart"]
```

For the logging component, esp supports both log and demt. log is used by default in this example. log is also currently recommended.

> 对于日志组件，esp 支持 log 和 defmt，这个例子中默认使用了 log。目前也推荐使用log。

```toml
[features]
default = [
    ...
    "log",
]

log = [
    "dep:log",
    "esp-hal/log",
    "esp-println/log",
    "esp-backtrace/println",
]

defmt = [
    "dep:defmt",
    "esp-hal/defmt",
    "esp-println/defmt-espflash",
    "esp-backtrace/defmt",
]
```

You can also change it to defmt, remember to change the rustflags in the `.cargo/config.toml` file.

> 你也可以修改为 defmt, 记得修改 `.cargo/config.toml` 文件中的 rustflags。

```toml
rustflags = [
  ...
  # with defmt
  "-C", "link-arg=-Tdefmt.x",
]
```

The reason why log is recommended over defmt is that defmt currently does not output backtrace correctly, and I don't know if this will be fixed in the future.

> 之所以推荐 log 而不是 defmt，因为 defmt 目前还不能正确输出 backtrace, 不知道未来能不能解决。

use log:

```sh
...
I (136) boot: Loaded app from partition at offset 0x10000
I (137) boot: Disabling RNG early entropy source...
hello world!
ERROR - this is error message
WARN - this is warn message
INFO - this is info message
loop!



!! A panic occured in 'src/main.rs', at line 52, column 9:
aa

Backtrace:

0x42002c60
0x42002c60 - hal_main
    at ??:??
0x42000104
0x42000104 - _start_rust
    at ??:??
```

use defmt:

```sh
...
I (136) boot: Loaded app from partition at offset 0x10000
I (136) boot: Disabling RNG early entropy source...
hello world!
loop!
�
 jsrc/main.rs@4	.aa�
�	~�
          �(JB~�
                JB~%
```
