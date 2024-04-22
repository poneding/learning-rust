[Learning Rust](../../README.md) / [rust-lang-book](../zz_generated_mdi.md) / [rlb01_hello-cargo](zz_generated_mdi.md) / Rust cargo 管理工具

# Rust cargo 管理工具

## 创建项目

```bash
cargo new rlb01_hello-cargo
cd rlb01_hello-cargo
```

> 可以使用 `cargo new --vcs git rlb01_hello-cargo` 创建项目并初始化 git 仓库，它将自动创建一个 .gitignore 文件。

## 编译项目

```bash
cargo build

# 编译之后将在 target/debug 目录下生成可执行文件
# 可以通过以下命令运行
./target/debug/rlb01_hello-cargo
```

## 运行项目

```bash
cargo run
```

## 检测项目是否可以编译

```bash
cargo check
```

## 安装可执行文件（更新）

```bash
cargo install --path .
```

## 卸载可执行文件

```bash
cargo uninstall rlb01_hello-cargo
```

## 发布项目

发布到 crates.io，需要注册账号。

并且，需要在 Cargo.toml 中添加部分内容，例如作者、描述、许可证必要信息以及其他信息：

```toml
[package]
name = "rlb01_hello-cargo"
version = "0.2.0"
edition = "2021"
authors = ["Pone Ding <poneding@gmail.com>"]
description = "A hello world program in Rust"
license = "MIT OR Apache-2.0"
readme = "README.md"
keywords = ["hello", "world"]
categories = ["hello-world"]
repository = "https://github.com/poneding/learning-rust/tree/master/rust-lang-book/rlb01_hello-cargo"
homepage = "https://github.com/poneding/learning-rust/tree/master/rust-lang-book/rlb01_hello-cargo"
```

```bash
cargo publish

# 忽略未提交的更改
cargo publish --allow-dirty
```

## 添加依赖

```bash
cargo add rand

cargo add rlb01_hello-cargo

# 添加本地依赖
cargo add rlb01_hello-cargo --path ../rlb01_hello-cargo
```

## 更新依赖

```bash
# 更新所有依赖
cargo update

# 更新指定依赖
cargo update rand
```

## 移除依赖

```bash
cargo rm rand
```

## 生成文档

```bash
cargo doc
```

## 测试

```bash
cargo test
```
