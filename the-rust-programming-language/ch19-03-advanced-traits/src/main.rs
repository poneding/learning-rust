// 默认泛型类型参数和运算符重载
use std::ops::Add;
fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Point2 { x: 1, y: 0 } + Point2 { x: 2, y: 3 },
        Point2 { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(1).add(Meters(1)), Millimeters(1001));

    let person = Human;
    person.fly(); // 将调用直接实现在类型上的方法

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name()); // Spot

    // <Type ad Trait>
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy

    let p3 = Point3 { x: 1, y: 1 };
    p3.outline_print();

    let wrapper = Wrapper(vec![String::from("Hello"), String::from("World!")]);
    println!("w: {wrapper}");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0 * 1000)
    }
}

// 完全限定语法与消歧义：调用相同名称的方法

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point3 {
    x: i32,
    y: i32,
}
impl OutlinePrint for Point3 {}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// newtype 模式用以在外部类型傻姑娘实现外部 trait

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}]", self.0.join(", "))
    }
}
