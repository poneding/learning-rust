#[allow(unused)]
fn main() {
    let i1 = 10;
    let i2 = i1; // 拷贝，i1 仍然有效。 类似的数据类型有：整数、浮点数、布尔值、字符、元组（只包含上述类型）
    println!("i1: {}", i1);

    let s1 = String::from("Hello");
    let s2 = s1; // 移动，s1 无效。 类似的数据类型有：String、Vec、Box、Fn
                 // println!("{}", s1); // 这里会报错，因为 s1 的所有权已经被转移

    // 当然，你也可以使用 clone() 方法来复制堆上的数据
    let s3 = s2.clone(); // 克隆，s2 仍然有效
    println!("s2: {}", s2);

    let i3: i32 = 20;
    make_copy(i3); // 传入的参数是整数，整数是 Copy 类型，所以 i3 仍然有效
    println!("i3: {}", i3);

    let s4 = String::from("Jay");
    take_ownership(s4); // s4 的所有权被转移给函数 take_ownership
                        // println!("{}", s4); // 这里会报错，因为 s4 的所有权已经被转移

    let s5 = give_ownership(); // 函数 give_ownership 返回一个 String 类型的值，s5 获得了这个值的所有权
    println!("s5: {}", s5);

    let s6 = String::from("Hello");
    let s7 = take_and_give_back(s6); // 函数 take_and_give_back 返回一个 String 类型的值，s6 的所有权被转移给函数，函数返回的值又转移给了 s7
    println!("s7: {}", s7);

    let s8 = String::from("Hello");
    take_reference(&s8); // 传入引用，不会转移所有权
    println!("s8: {}", s8);

    let mut s9 = String::from("Hello");
    take_reference_and_mut(&mut s9); // 传入可变引用，可以修改
    println!("s9: {}", s9);

    let mut s10 = String::from("Hello");
    // let r1 = &mut s10;
    // let r2 = &mut s10; // 为了避免数据竞争，Rust 不允许同时存在多个可变引用
    // println!("r1: {}, r2: {}", r1, r2);

    let r1 = &s10;
    let r2 = &s10; // 允许存在多个不可变引用
                   // println!("r1: {}, r2: {}", r1, r2);
    let r3 = &mut s10;
    // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3); // 这里会报错，不能同时存在可变引用和不可变引用
    println!("r3: {}", r3);

    // 但是在使用引用后，可以再声明一个可变引用
}

fn make_copy(i: i32) {
    println!("{}", i);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello");
    s // 返回 s，s 的所有权转移给调用者
}

fn take_and_give_back(s: String) -> String {
    s // 返回 s，s 的所有权转移给调用者
}

// 不可变引用，不会转移所有权，并且不可修改
fn take_reference(s: &String) {
    println!("{}", s);
}

// 可变引用，可以修改，但是传入的参数必须是可变的
fn take_reference_and_mut(s: &mut String) {
    s.push_str(", World!");
    println!("{}", s);
}
