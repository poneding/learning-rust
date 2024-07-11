[Learning Rust](README.md) / Rust 学习笔记

# Rust 学习笔记

## 测试

`mod` 上方的 `cfg(test)` 注解表示该模块仅在测试时编译。

函数上方的 `#[test]` 注解表示该函数是一个测试函数，通过 `#[ignore]` 注解可以忽略该测试。

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
