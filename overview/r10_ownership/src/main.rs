fn main() {
    // 变量需要负责释放他们负责的资源，所以资源只能拥有一个所有者。
    // 引用变量不拥有资源
    // os1();
    // os2();

    os3();
    // 所有权只会发生在堆上分配的数据，基础类型
    // （整型，浮点型，布尔，字符char，仅包含基础类型的元组）存储在栈上
    // 基础类型值拷贝

    os_clone()
}

#[allow(unused)]
// os: ownership
fn os1() {
    // 栈上分配
    let a = 1;
    let b = a; // copy 操作，不存在资源移动
    println!("a: {}, b: {}", a, b); //

    // 堆上分配
    // let v1 = vec!["Vue", "Rust", "Golang"]; // 此时 v1 拥有堆上数据的所有权
    // let v2 = v1; // 至此，v1 失去了堆上数据的所有权，v2 获得所有权
    // println!("{:?}", v1) // 报错，borrow of moved value: `v1`
}

#[allow(unused)]
fn os2() {
    let v1 = vec!["Vue", "Rust", "Golang"];
    let v2 = v1;
    show_v(v2); // show_v 中参数v 获得堆上数据所有权，v2 失去所有权
                // println!("v2: {:?}", v2) // 报错，borrow of moved value: `v2`
}

#[allow(unused)]
fn show_v(v: Vec<&str>) {
    println!("v: {:?}", v)
}

fn os3() {
    let v1 = vec!["Vue", "Rust", "Golang"];
    let v2 = v1;
    let result = show_v_2(v2);
    println!("result: {:?}", result) // 不会报错
}

fn show_v_2(v: Vec<&str>) -> Vec<&str> {
    println!("v: {:?}", v);
    return v;
}

// 将数据克隆一份，给其他变量使用，自己也不丢失所有权
fn os_clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    let mut s3 = s2.clone();
    s3.push_str(", Rust");
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
    // println!("s1: {}, s2: {}", s1, s2);
}
