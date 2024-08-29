fn main() {
    m1();
    println!("----------------");
    m2();
    println!("----------------");
    m3();
    println!("----------------");
    m4();
    println!("----------------");
    m5();
    println!("----------------");
    m6();
    println!("----------------");
    m7();
    println!("----------------");
    m8();
    println!("----------------");
    m9();
}

// 模式语法

// 1. 匹配字面值
fn m1() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), // _ 匹配其他
    }
}

// 2. 匹配命名变量
fn m2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // 这里 y 是一个新变量了
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end, x = {x:?}, y = {y}"); // 所以这里 y 还是 10
}

// 3. 多个模式
fn m3() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // 使用 ｜（or） 匹配多个模式
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 4. 通过 .. 或者 ..= 匹配范围
fn m4() {
    let x = 5;

    match x {
        1..3 => println!("one or two"),          // 1,2
        3..=5 => println!("three through five"), // 3,4,5
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// 5. 解构并分解值
fn m5() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(a, 0);
    assert_eq!(b, 7);

    let Point { x, y } = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"), // y 为 0，在 x 轴上
        Point { x: 0, y } => println!("On the y axis at {y}"), // x 为 0，在 y 轴上
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})") // 不在轴线上
        }
    }
}

// 6. 解构枚举
#[allow(unused)]
fn m6() {
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
    }

    // 解构嵌套的结构体和枚举
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }
}

// 7. 使用 .. 忽略剩余值
#[allow(unused)]
fn m7() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}"); // Some numbers: 2, 32
        }
    }
}

// 8. 匹配额外条件
fn m8() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"), // matched
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"), // matched
    }
}

// 9. @ 绑定
fn m9() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"), // matched
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
