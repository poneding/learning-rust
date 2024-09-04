fn main() {
    // 类型别名
    type Kilometers = i32;

    let x = 1;
    let y: Kilometers = 1;
    assert_eq!(x + y, 2);

    // 通过类型别名减少编写
    type Thunk = Box<dyn Fn() + Send + 'static>;
    fn takes_long_type(f: Thunk) {
        f();
    }
    fn returns_long_type() -> Thunk {
        Box::new(|| println!("Hello!"))
    }
    let f: Thunk = returns_long_type();
    takes_long_type(f);
}
