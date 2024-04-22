use std::fmt;
#[allow(unused)]
fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    let result = largest(&nums);
    println!("The largest number is {}", result);

    let strings = vec!["hello", "world", "foo", "bar"];
    let result = largest(&strings);
    println!("The largest string is {}", result);

    let p1 = Point { x: 5, y: 10 };
    let p11 = Point::new(5, 10);
    let p2: Point<f32> = Point { x: 1.0, y: 4.0 };

    let p3 = Point2 { x: 5, y: 10.4 };

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
}

// 泛型 T 的限制：1 可以拷贝；2 可以排序
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[allow(unused)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    // fn print(&self) {
    //     println!("point: {}", self);
    // }
}

#[allow(unused)]
struct Point2<T, U> {
    x: T,
    y: U,
}
