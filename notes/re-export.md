# 重新导出

```rust
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

之后，可以在外部可以直接使用如下方式引用：

```rust
use art::PrimaryColor;
use art::SecondaryColor;
use art::mix;
```
