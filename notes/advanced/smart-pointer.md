# 智能指针

## 实现 Deref trait

Deref trait 是一个特殊的 trait，它允许我们重载解引用运算符 `*`。

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

## 实现 Drop trait

Drop trait 是一个特殊的 trait，

```rust
struct MyBox<T>(T);

impl<T> Drop for MyBox<T> {
    // 在 MyBox 变量离开作用域时，将执行
    fn drop(&mut self) {
        println!("droped mybox!");
    }
}
```
