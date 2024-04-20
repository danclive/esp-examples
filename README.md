# ESP Examples

Some examples of [esp-hal](https://github.com/esp-rs/esp-hal).

> 一些使用 [esp-hal](https://github.com/esp-rs/esp-hal) 的例子：

- [x] [blinky](blinky): Blinks an LED and output logs
- [x] [embassy_blinky](embassy_blinky): use embassy to blinks an LED and output logs
- [x] [embassy_wifi](embassy_wifi): use embassy to connect to wifi

Unlike the examples that come with esp-hal, each example is separate and handles dependencies, so you can just copy an example to get a quick start on your project.

> 不同于 esp-hal 自带的示例，这里每一个示例都是单独的、处理好依赖的，你可以直接复制某个示例快速开始项目。

You can check the README.md file in each example for more information.

> 你可以查看每个示例中的 README.md 文件了解更多信息。

But, I only have the esp32c3 and esp32c6 development boards at the moment, these examples are only tested the esp32c3 and esp32c6, maybe in the future I'll add more chip models. For other chip models, you may have to make some changes to the Cargo.toml file and the .cargo/config.toml file.

> 但是，目前我只有 esp32c3 和 scp32c6 的开发板，这些示例只在这两个型号上进行了测试，也许未来会添加更多芯片型号。对于其他芯片型号，你可能需要修改 Cargo.toml 文件和 .cargo/config.toml 文件。
