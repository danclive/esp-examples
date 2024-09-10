# Rhai

[Rhai](https://rhai.rs/) is a tiny, simple and fast embedded scripting language for Rust that gives you a safe and easy way to add scripting to your applications.

> [Rhai](https://rhai.rs/)是 Rust 的一种小巧、简单而快速的嵌入式脚本语言，它为你提供了一种安全而简便的方法来为你的应用程序添加脚本。

This example is modified from [alloc](../alloc). If you don't understand something, you can read the contents of [alloc](../alloc) before continuing on.

> 这个例子是在 [alloc](../alloc) 的基础上修改而来的。如果你有不懂的地方，你可以先阅读 [alloc](../alloc) 的内容，再继续往下阅读。


要使用 rhai，只需要在 `Cargo.toml` 文件中添加以下内容：

> To use rhai, simply add the following to the `Cargo.toml` file:

```toml
default = [
    ...
    "rhai"
]

rhai = [
    "dep:rhai",
    "rhai/f32_float",
    "rhai/only_i32"
]

[dependencies]
rhai = { version = "1.19", default-features = false, features = ["no_std"], optional = true }

[profile.release]
lto = "fat"         # turn on Link-Time Optimizations
codegen-units = 1   # trade compile time with maximum optimization
opt-level = "z"     # optimize for size
```

In order to reduce the size of the binary obtained by compiling, here are some settings, you can refer to [Minimal Build](https://rhai.rs/book/start/builds/minimal.html).

> 为了减小编译得到的二进制大小，这里做了一些设置，你可以参考 [Minimal Build](https://rhai.rs/book/start/builds/minimal.html)。

For more details on how to use Rhai, you can check out the `main.rs` file and check out the [Rhai](https://rhai.rs/) documentation, which only gives a very basic demonstration.

> 关于 Rhai 具体的使用方法，你可以查看 `main.rs` 文件并查看 [Rhai](https://rhai.rs/) 文档，这里只做了最基本的演示。

Note that Rhai dynamically allocates content, so set a sufficient heap size.

> 要注意的是，Rhai 会动态分配内容，请设置足够的堆大小。

```rust
fn init_heap() {
    const HEAP_SIZE: usize = 128 * 1024;
    ...
}
```
