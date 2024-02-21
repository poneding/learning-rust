fn main() {
    hello();
    let res = add();
    println!("res: {}", res);

    let s = get_string();
    println!("s: {}", s);

    let s2 = get_string2();
    println!("s2: {}", s2);

    let price = 99;
    double_price(price);
    println!("price out fn double_price is: {}", price);

    let mut price2 = 199;
    double_price2(&mut price2);
    println!("price2 out fn double_price2 is: {}", price2);
    // greet();
}

fn hello() {
    println!("Hello Rust!");
}

fn add() -> i32 {
    return 1 + 1;
}

fn get_string() -> String {
    return String::from("Jay");
}

fn get_string2() -> String {
    // 取最后一条语句的执行结果，必须和函数返回值数据类型保持一致。
    let num = 3;
    if num > 4 {
        return String::from("Jay");
    }
    String::from("Ding") // 而且最后没有分号表示返回值
}

// 参数 值传递
fn double_price(mut price: i32) {
    price = price * 2;
    println!("price in fn double_price is: {}", price);
}

// 参数 引用传递
fn double_price2(price2: &mut i32) {
    // 引用传递在类型前添加 & 符号
    *price2 = *price2 * 2; // * 号用于访问引用地址指向的内存位置上变量的值，称为解引用
    println!("price2 in fn double_price2 is: {}", price2);
}

// 复合类型如字符串String类型，作为参数传递给函数后，该变量不可再访问
fn show_name(name: String) {
    println!("Hello, {}", name)
}

fn greet() {
    let name: String = String::from("Jay");
    show_name(name);
    println!("after show_name: {}", name) //error[E0382]: borrow of moved value: `name`
}
