fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("area: {}", area_1(width1, height1));

    // 元组类型
    let rect1 = (30, 50);
    println!("area: {}", area_2(rect1));

    // 结构
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area: {}", area_3(&rect2));

    // 派生 trait 打印 Rectangle
    println!("rect2: {:?}", rect2);
    println!("rect2: {:#?}", rect2);
    println!("rect2: {rect2:?}"); // 单行
    println!("rect2: {rect2:#?}"); // 多行结构
    dbg!(&rect2);

    // dbg! 可以接收一个表达式的所有权，print! 接收引用
    dbg!(1 + 1); // 打印代码文件路径以及 1 + 1 = 2
    println!("1 + 1 = {}", 1 + 1); // 字符串字面值
}

fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 派生 trait 打印 Rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
