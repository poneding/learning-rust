// 方法与函数：
// 为结构体实现方法，第一个参数为 &self
// 不为结构体实现的为函数，例如 main 函数
fn main() {
    let r1 = Rectangle {
        width: 30,
        heigth: 50,
    };
    let r2 = Rectangle {
        width: 25,
        heigth: 40,
    };
    let r3 = Rectangle {
        width: 40,
        heigth: 40,
    };

    println!("r1 can hold r2: {}", r1.can_hold(&r2));
    println!("r1 can hold r3: {}", r1.can_hold(&r3));
}

#[derive(Debug)]
#[allow(unused)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

// 函数
#[allow(unused)]
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.heigth
}

// 方法
#[allow(unused)]
impl Rectangle {
    // 方法第一个参数是 &self
    // &self 实际上是 self: &Self 的缩写
    // 如果要获取实例的所有权，使用 &mut self
    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    // 方法可以与解构字段同名
    fn width(&self) -> bool {
        self.width > 0
    }

    // 实例的指针也可以直接调用方法，因为 Rust 会自动引用和解引用（auto ref and deref）

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.heigth > other_rect.heigth
    }
}

// 同一个结构体可以拥有多个 impl 实现块
#[allow(unused)]
impl Rectangle {
    // 关联函数，在 impl 块中定的函数。这类也被称为函数，因为第一个参数不是 &self
    // 返回一个正方行，使用 Rectangle::square() 调用
    fn square(width: u32) -> Self {
        Self {
            width,
            heigth: width,
        }
    }
}
