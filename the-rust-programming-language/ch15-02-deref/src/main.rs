use std::ops::Deref;

fn main() {
    let x = 5;
    let my_box = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *my_box);
    // *my_box 在底层相当于执行了 *(my_box.deref())

    // 隐式 Deref 强制转换
    let my_box2 = MyBox::new(String::from("Rust"));
    hello(&my_box2);
}

// 隐式 Deref 强制转换
fn hello(s: &str) {
    println!("Hello, {}", s);
}

// 实现自己的智能指针：MyBox<T>
// 元组
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
