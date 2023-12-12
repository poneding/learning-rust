fn main() {
    // Vec: 也可以叫切片，指向一段连续内存的指针。
    // 运行时才知道长度大小。
    let mut v = Vec::new();
    v.push("Vue");
    v.push("Rust");
    v.push("Golang");
    println!("{:?}", v);
    println!("len of v: {}", v.len());
    let s1 = &v[1..3];
    println!("{:?}", s1);

    show_vec(s1);

    let mut v2 = Vec::new();
    v2.push("Vue");
    v2.push("Rust");
    v2.push("Golang");
    println!("before mut_vec() v2: {:?}", v2);
    mut_vec(&mut v2);
    println!("after mut_vec() v2: {:?}", v2);
}

fn show_vec(s: &[&str]) {
    println!("{:?}", s);
}

fn mut_vec(s: &mut [&str]) {
    s[0] = "Vue3";
    println!("in mut_vec s: {:?}", s);
}


