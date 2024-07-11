fn main() {
    // 拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    // fn add(self, s: &str) -> String
    // &String 可以通过 Deref 强转为 &str：&s2 => &s2[..]
    let hello_world = s1 + &s2; // s1 被移动，不能继续使用，s2 借用
    println!("{hello_world}");

    let s3 = "Hello";
    let hello_world2 = format!("{s3}, {s2}");
    println!("{hello_world2}");

    for c in "Зд".chars() {
        println!("{c}");
        // З
        // д
    }

    for b in "Зд".bytes() {
        println!("{b}");
        // 208
        // 151
        // 208
        // 180
    }
}
