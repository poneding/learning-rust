fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };
    println!("r: {:?}", r);
    println!("area of r: {}", r.area());

    let square = Rectangle::square(80); // 使用关联函数创建正方形
    println!("square: {:?}", square);
    println!("area of square: {}", square.area());

    let cq = square.can_hold(&r);
    println!("Can square hold r? {}", cq);
}

#[derive(Debug)]
#[allow(unused)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
