# 测试

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

## 单元测试

单元测试独立地验证库的不同部分，也能够测试私有函数实现细节。

每个文件下创建包含测试函数的 `tests` 模块，并使用 `#[cfg(test)]` 注解。

文件：*src/lib.rs*

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

## 集成测试

集成测试检查多个部分是否能结合起来正确地工作，并像其他外部代码那样测试库的公有 API。

创建与 `src` 同级的 `tests` 目录，然后在该目录下创建测试文件。每一个测试文件都会被 cargo 编译为一个独立的 crate。

结构：

```txt
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

文件：*tests/integration_test.rs*

```rust
use adder::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
```

集成测试子模块：

```txt
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

文件：*tests/common/mod.rs*

```rust
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

文件：*tests/integration_test.rs*

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```
