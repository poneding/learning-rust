#![allow(unused)]
use std::collections::HashMap;
fn main() {
    // let a = String::from("Hello");
    // let b = String::from("World");

    // let mut m1: HashMap<String, i32> = HashMap::new();
    // m1.insert(a, 1);
    // m1.insert(b, 1);

    // for (k, v) in m1 {
    //     println!("{}: {}", k, v)
    // }

    // let v1 = m1.get(&String::from("Hello")); // Option
    // let v1 = v1.expect("Hello not found");
    // print!("{}", v1);

    // 更新
    // let mut m2: HashMap<String, i32> = HashMap::new();
    // m2.insert(String::from("Hello"), 1);
    // m2.insert(String::from("Hello"), 2);

    // // entry 获取键对应的值并继续执行
    // m2.entry(String::from("Jay")).or_insert(1); // 如果不存在则插入
    // m2.entry(String::from("Jay")).or_insert(2); // 已经存在，不会插入或更新

    // for (k, v) in m2 {
    //     println!("{}: {}", k, v)
    // }

    // 统计单词出现次数
    // let txt = "Hello Jay Jay is an Musician and an Director";
    // let mut m3: HashMap<&str, i32> = HashMap::new();
    // for w in txt.split_whitespace() {
    //     let c = m3.entry(w).or_insert(0);
    //     *c += 1;
    // }

    // for (k, v) in m3 {
    //     println!("{}: {}", k, v)
    // }
}
