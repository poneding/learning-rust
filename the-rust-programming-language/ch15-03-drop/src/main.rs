// Drop trait: 当直再离开作用域时执行 drop 方法，该方法一般用于释放资源
// Box<T> 被丢弃时，会释放 box 指向的堆空间

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    // _c.drop(); // 不能显示调用 drop 方法
    // 但是可以使用 drop 函数调用
    drop(_c); // 本来变量时以创建相反的顺序被丢弃，但这里显示的 drop c_，所以 _c 会先被丢弃。
    println!("CustomSmartPointer dropped before the ene of main.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // drop 将在 CustomSmartPointer 变量离开作用域时被调用
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}
