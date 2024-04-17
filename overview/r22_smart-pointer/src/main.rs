use std::ops::Deref;

fn main() {
    box1();
    box2();
    deref1();
}

// 指针封装在两个 Trait
// - Deref 用于创建一个只读智能指针，例如 *v
// - Drop 智能指针超出它的作用域返回时调用该 Trait 的 drop() 方法，类似于
// 析构函数

// 当一个结构体实现了以上的接口后，它就不是普通结构体

// Box 指针 可以把数据存储在堆上，而不是栈上。
// 这就是装箱 box，栈 stack 还是包含指向堆上数据的指针
fn box1() {
    let a = 100; // 栈上
    let b = Box::new(a); // 数据在堆上，指针在栈上
    println!("b: {}", b);

    println!("100=a? {}", 100 == a);
    // 访问 Box 存储的数据，使用 *，解引用
    println!("*b=100? {}", 100 == *b); // *b 是一个智能指针（解引用），指向堆上的数据
}

// 修改 Box 存储的数据，使用 Deref 的 *mut 子接口，解引用
fn box2() {
    let mut c = Box::new(100);
    *c = 200;
    println!("c: {}", c);
    let c2 = &c;
    println!("c2: {}", c2);

    let mut d = Box::new(100);
    println!("d: {}", d);
    d = Box::new(200);
    println!("d: {}", d);
}

struct CustomBox<T> {
    value: T,
}

impl<T> CustomBox<T> {
    fn new(value: T) -> CustomBox<T> {
        CustomBox { value }
    }
}

// 实现 Deref deref() 将 Box 存储的数据解引用
impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

fn deref1() {
    let ca = 200;
    let cb = CustomBox::new(ca); // 堆上

    println!("*cb: {}", *cb);
    println!("200==*cb? {}", 200 == *cb);
}

// 实现 Drop drop() 在 Box 存储的数据超出它的作用域时调用该方法，类似于析构函数
impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        println!("drop CustomBox");
    }
}
