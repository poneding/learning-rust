[Learning Rust](../README.md) / [rust-lang-book](zz_generated_mdi.md) / Rust 程序设计语言

# Rust 程序设计语言

中文文档：[Rust 程序设计语言](https://rustwiki.org/zh-CN/book/)

## 安装

Linux & Mac：

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

一些常用的 Rust 包依赖于 C 代码，因此可能需要额外安装 C 编译器，在 Mac 上通过运行以下命令可以获得 C 编译器：

```bash
xcode-select --install
```

Ubuntu 上通过运行以下命令可以获得 C 编译器：

```bash
sudo apt install build-essential
```

## 更新

```bash
rustup update
```

## 卸载

```bash
rustup self uninstall
```
