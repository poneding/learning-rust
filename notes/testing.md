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
