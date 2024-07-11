#[allow(unused)]
fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // read
    let third = &v[2];
    println!("The third element is {third}"); // 返回一个 i32，如果超出数组，程序 panic

    let third = v.get(2); // 返回一个 Option<i32>
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i *= 10;
    }

    for i in &v {
        println!("{i}");
    }

    // vector 只能存储相同类型的值，有时候可能不太方便。
    // 我们可以使用枚举作为存储元素
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello")),
    ];
}
