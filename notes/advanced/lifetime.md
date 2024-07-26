# 生命周期

生命周期属于一类特殊的泛型。在 Rust 中，生命周期是一种标记引用有效性的方式。生命周期的主要作用是避免悬垂引用。

> 悬垂引用是指引用的对象已经被销毁，但引用仍然存在。

## 生命周期注解

生命周期注解是一种标记引用有效性的方式。生命周期注解的语法是一个单引号 `'` 后跟生命周期名字，如 `'a`。示例：

```rust
&'a i32     // 带有显示生命周期注解的引用
&'a mut i32 // 带有显示生命周期注解的可变引用
```

## 函数签名中的生命周期注解

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 结构体中的生命周期注解

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

## 方法中的生命周期注解

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## 生命周期省略

三条规则：

1. 编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个引用参数的就有一个生命周期参数：`fn foo<'a>(x: &'a i32)`，有两个引用参数的函数就有两个不同的生命周期参数，`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，依此类推；
2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`；
3. 如果方法有多个输入生命周期参数并且其中一个参数是 `&self` 或 `&mut self`，说明是个对象的方法 (method)，那么所有输出生命周期参数被赋予 `self` 的生命周期。

> 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。

## 静态生命周期

`'static` 是一个特殊的生命周期，其生命周期存活于整个程序期间。所有的字符串字面值都拥有 `'static` 生命周期。

```rust
let s: &'static str = "I have a static lifetime.";
```
