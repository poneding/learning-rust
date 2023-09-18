# learning-rust

## 开发环境

安装：

```bash
# 安装 rustup，rustup 是 rust 安装器以及版本管理工具
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- VSCode 安 装rust-analyzer 插件
- Goland 安装 Rust 插件、Native Debugging Support 等插件

升级：

```bash
# 检查更新
rustup check

# 更新
rustup update
```

## 创建项目

```bash
# 创建项目，将自动生成一个项目目录
cargo new r01_hello-world

# 在目录中创建项目
cargo init
```

## 引入依赖

crates.io 是 Rust 的包管理器，类似于 npm

```bash
cargo add <package-name>@<version>
```

## 编译项目

```bash
# 编译项目
cargo build

# 编译并运行
cargo run
```

## 生成文档

```bash
cargo doc
```

## 发布项目

发布到 crates.io

```bash
cargo publish
```

## 测试

```bash
cargo test
```