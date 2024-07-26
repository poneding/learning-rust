# 格式化打印

- 实现 `std::fmt::Display` trait，实现 `fmt` 方法，可以使用 `{}` 占位符。
- 使用 `#[derive(Debug)]` 注解，实现 `std::fmt::Debug` trait，可以使用 `{:?}` 或 `{:#?}` 占位符。

> 实现了 `std::fmt::Display` trait 的类型，自动实现了 ToString trait。实例可以通过使用 `to_string` 方法转换为字符串。

```rust
use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 10, y: 20 };
    println!("{}", point); // 正常输出: (10, 20)
    println!("{:?}", point); // 编译错误: `Point` doesn't implement `Debug`
}
```
