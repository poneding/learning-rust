[Learning Rust](README.md) / Rust 学习笔记

# Rust 学习笔记

## 格式化打印

- 实现 `std::fmt::Display` trait，实现 `fmt` 方法，可以使用 `{}` 占位符。
- 使用 `#[derive(Debug)]` 注解，实现 `std::fmt::Debug` trait，可以使用 `{:?}` 或 `{#?}` 占位符。

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

## 重导出

```rust
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

## 测试

1、`mod` 上方的 `cfg(test)` 注解是为了让编译器仅在 cargo test 时才编译，cargo build 时无需编译。

2、函数上方的 `#[test]` 注解表示该函数是一个测试函数，通过 `#[ignore]` 注解可以忽略该测试。

3、#[should_panic] 注解表示该测试函数应该 panic，可以指定 panic 的信息，如 `#[should_panic(expected = "oops")]`。

```rust
cargo test

# 指定线程数
cargo test -- --test-threads=1

# 测试不捕获 stdout/stderr，而是允许直接打印输出
cargo test -- --nocapture

# 打印测试捕获到的 stdout
cargo test -- --show-output

# 运行指定测试
cargo test <test_fn_name>

# 运行被忽略的测试
cargo test -- --ignored

# 运行测试包括忽略的测试
cargo test -- --include-ignored

# 运行所有测试
cargo test --all
```
