[Learning Rust](../README.md) / [the-rustonomicon](zz_generated_mdi.md) / the-rustonomicon

# the-rustonomicon

使用 unsafe 关键字，在其作用域内允许编写不安全的 rust 代码。

禁止写 unsafe 的代码：在代码作用域中引入 `#![forbid(unsafe_code)]` 注解，这样编译器会拒绝 unsafe 代码。
