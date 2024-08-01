# Rust 标准代码结构

## Cargo Target

- Binary：二进制对象
- Library：库对象
- Example：示例对象
- Test：测试对象
- Bench：基准测试对象

```txt
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

- 默认二进制入口文件是 `src/main.rs`，库入口文件是 `src/lib.rs`。
- 其他二进制文件放在 `src/bin/` 目录下
- 基准测试文件放在 `benches/` 目录下
- 示例代码放在 `examples/` 目录下
- 集成测试文件放在 `tests/` 目录下

## 多二进制对象运行

```sh
# 运行默认二进制对象
cargo run --bin <crate-name>

# 运行指定二进制对象
# 例如：
# cargo run --bin named-executable
# cargo run --bin another-executable
# cargo run --bin multi-file-executable
cargo run --bin <bin-name>
```
