fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传入 String 的引用，避免所有权被移动
                                     // 创建一个引用的行为称之为借用

    println!("The length of '{s1}' is {len}");

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2: {s2}");

    // 无法同时创建两个可变引用后，并继续使用
    // let mut s3 = String::from("hello");
    // let s4 = &mut s3;
    // let s5 = &mut s3;
    // println!("s4: {s4}, s5: {s5}");

    // 也无法在创建可变引用后，继续使用之前的不可变引用
    // let mut s6 = String::from("hello");
    // let s7 = &s6;
    // let s8 = &mut s6;
    // println!("s7: {s7}, s8: {s8}");

    // 总之：
    // - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // - 引用必须总是有效的。
}

fn calculate_length(s: &String) -> usize {
    s.len() // 这行结束后，s 离开了作用域，但是 s 是一个引用，并不拥有所有权
}

fn _change(_s: &String) {
    // _s.push_str(", world"); // _s 为一个不可变应用，因此这里无法通过编译
}

fn change(s: &mut String) {
    s.push_str(", world");
}
