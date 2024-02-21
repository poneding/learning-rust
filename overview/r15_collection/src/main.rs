use std::collections::{HashMap, HashSet};

fn main() {
    // 向量 Vector
    // 哈希表 HashMap
    // 哈希集合 HashSet

    vec1();
    hashmap();
    hashset();

    iterator_next();
    iterator_iter();
    iterator_into_iter();
    iterator_iter_mut();
}

// 向量 Vector（Vec），在别的语言中可能叫做切片 slice
// - 相同元素的集合
// - 长度可变
// - 内存在堆上
fn vec1() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    // v.push("hello");// 会报错
    println!("v: {:?}", v);
    println!("v[0]: {}", v[0]);

    // 通过 vec! 宏创建向量
    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);
    let i = v2.remove(0);
    println!("i: {:?}", i);
    println!("v2: {:?}", v2);

    if v2.contains(&5) {
        println!("v2 contains 5");
    }

    for item in v2 {
        println!("item: {}", item);
    }
}

fn hashmap() {
    let mut h = HashMap::new();
    h.insert("a", 1);
    h.insert("b", 2);
    println!("{:?}", h);
    println!("h.len: {}", h.len());

    match h.get("a") {
        // match h.get("c") {
        Some(v) => {
            println!("h.get v: {}", v);
        }
        None => {
            println!("h.get v: None");
        }
    }

    for (k, v) in h.iter() {
        println!("k: {}, v: {}", k, v);
    }

    if h.contains_key("a") {
        println!("h contains key a");
    }

    let val = h.remove("a");
    println!("val: {:?}", val);
    println!("h: {:?}", h);
}

fn hashset() {
    let mut hs = HashSet::new();
    hs.insert("Vue");
    hs.insert("Golang");
    hs.insert("Rust");
    println!("{:?}", hs);
    println!("hs.len: {}", hs.len());

    for item in hs.iter() {
        println!("item: {}", item);
    }

    if hs.contains(&"Vue") {
        println!("hs contains Vue");
    }

    if hs.remove(&"Vue") {
        println!("hs remove Vue");
    }
}

// 迭代器
// iter() 返回一个只读可重入迭代器 元素类型为 &T
// into_iter() 返回一个只读不可重入迭代器，迭代器元素类型为 T
// iter_mut() 返回一个可写可重入迭代器，迭代器元素类型为 &mut T
fn iterator_next() {
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next()); // None
}

fn iterator_iter() {
    let v = vec![1, 2, 3, 4, 5];
    for item in v.iter() {
        println!("item: {}", item);
    }
    println!("v after iter(): {:?}", v);
}

fn iterator_into_iter() {
    let ref v = vec![1, 2, 3, 4, 5];
    for item in v.into_iter() {
        println!("item: {}", item);
    }
    println!("v after into_iter(): {:?}", v);
}

fn iterator_iter_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    for item in v.iter_mut() {
        *item *= *item;
    }
    println!("v after iter_mut(): {:?}", v);
}
