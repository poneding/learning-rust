fn main() {
    let nums = vec![1, 2, 5, 3, 4];

    let largest = largest(&nums);
    println!("largest: {}", largest);

    let integer = Point { x: 1, y: 1 };
    println!("{:#?}", integer);
    println!("integer.x(): {}", integer.x());

    let float = Point { x: 1.0, y: 1.0 };
    println!("{:#?}", float);
    println!(
        "float.distance_from_origin(): {}",
        float.distance_from_origin()
    );
}

// 泛型函数
fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

// 泛型结构
#[derive(Debug)]
#[allow(unused)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 泛型枚举
enum _Option<T> {
    Some(T),
    None,
}

enum _Result<T, E> {
    Ok(T),
    Err(E),
}
