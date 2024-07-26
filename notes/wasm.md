# Rust WASM 编程

## 1. 初始化项目

```bash
cargo new hello-wasm
cd hello-wasm
```

## 2. 安装 wasm-pack

```bash
cargo install wasm-pack
```

## 3. 编写代码

编辑 *src/main.rs* 文件：

```rust
// 使用 wasm-bindgen 在 Rust 与 JavaScript 之间通信
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("Hello, {}!",name));
}
```

编辑 *Cargo.toml* 文件：

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
edition = "2021"
authors = ["Pone Ding <poneding@gmail.com>"]
description = "A sample project with wasm-pack"
license = "MIT/Apache-2.0"
repository = "https://github.com/poneding/hello-wasm"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

## 4. 构建项目

```bash
wasm-pack build --scope [npm-username]
```
