# Alloc

After setting up the heap allocator, we can use the types of collections, containers, etc., such as String, Vec, BTreeMap, Box, etc. in [alloc](https://doc.rust-lang.org/stable/alloc/index.html).

> 设置堆分配器后，我们就可以使用 [alloc](https://doc.rust-lang.org/stable/alloc/index.html) 中的集合、容器等类型，例如 String，Vec，BTreeMap，Box 等。

This example is modified from [blinky](../blinky). If you don't understand something, you can read the contents of [blinky](../blinky) before continuing on.

> 这个例子是在 [blinky](../blinky) 的基础上修改而来的。如果你有不懂的地方，你可以先阅读 [blinky](../blinky) 的内容，再继续往下阅读。

To set up the heap allocator, we need to add the following to `Cargo.toml`:

> 设置堆分配器，我们需要在 `Cargo.toml` 添加以下内容：

```toml
[features]
default = [
    ...
    "alloc"
]

alloc = [
    "esp-alloc",
    ...
]

[dependencies]
...
esp-alloc = { version = "0.5", optional = true }
```

and add it to the `main.rs` file:

> 并在 `main.rs` 文件中添加：

```rust
extern crate alloc;

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
    ...
}
```

Note that `HEAP_SIZE` needs to be sized appropriately for your application; and the microcontroller memory is small enough that you need to be very delicate with it.

> 要注意的是，`HEAP_SIZE` 需要根据你的应用去调整合适的大小；并且单片机内存很小，你需要很精细地使用内存。

Then you can use the types in [alloc](https://doc.rust-lang.org/stable/alloc/index.html):

> 然后就能使用 [alloc](https://doc.rust-lang.org/stable/alloc/index.html) 中的类型了：

```rust
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
```

If you want to use json, you also need to add it to `Cargo.toml`:

> 如果需要使用 json，还需要在 `Cargo.toml` 添加：

```toml
alloc = [
    ...
    "serde/alloc",
    "serde_json/alloc"
]

[dependencies]
...
serde = { version = "1.0", default-features = false, features = ["derive"], optional = true }
serde_json = { version = "1.0", default-features = false, optional = true }
```

Then it's ready to use:

> 然后就能使用了：

```rust
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// json
// see: https://docs.rs/serde_json/1.0.116/serde_json/
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
```

Along those lines, it's a similar approach for other crates that support `no_std`.

> 顺着这个思路，对于其他支持 `no_std` 的库，也是类似的做法。
