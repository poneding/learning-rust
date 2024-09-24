#![allow(unused)]
fn main() {
    let num = Number::from(10i32);
    println!("num: {:?}", num);

    // 如果类型实现了 From，那么也就免费获得了 Into
    let num: Number = 20i32.into(); // 但是这里需要显示的指定类型
    println!("num: {:?}", num);

    let num: Number2 = 15i32.into();
    println!("num: {:?}", num);
}

/// 为自定义类型实现转换
///
/// 实现 std::convert::From

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
struct Number2 {
    value: i32,
}

impl Into<Number2> for i32 {
    fn into(self) -> Number2 {
        Number2 { value: self }
    }
}
