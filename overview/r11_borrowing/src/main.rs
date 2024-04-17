fn main() {
    // &变量 借用出去，但是默认不可变
    let v1 = vec!["Vue", "Rust", "Golang"];
    let v2 = v1;
    show(&v2);
    println!("v2: {:?}", v2); // 不会报错

    // show2(&v2);
    // println!("v2: {:?}", v2); // 会报错，需要设置可变 mut 才能生效

    let mut v3 = vec!["Vue", "Rust", "Golang"];
    show3(&mut v3);
    println!("after show3 v3: {:?}", v3); // 不会报错
}

fn show(v: &Vec<&str>) {
    println!("v: {:?}", v);
}

// fn show2(v: &Vec<&str>) {
//     v[0] = "Java";
//     println!("v: {:?}", v);
// }

fn show3(v: &mut Vec<&str>) {
    v[0] = "Java";
    println!("in show3 v: {:?}", v);
}

// 如果要在借用（borrowing）的时候改变变量的值：
// 1 变量要用 mut 关键字
// 2 函数参数为可变的要用 mut 关键字
// 3 传递参数的时候，也要使用 mut 关键字
