use std::io; // 引入标准库 io，用于输入输出
use std::io::Write; // 引入标准库 io::Write Trait，用于输出

fn main() {
    println!("Hello, world!");

    let mut output = io::stdout(); // 获取标准输出并绑定到变量 output，io::output() 获取到的是一个实现了 Write Trait 的对象
    output.write(b"Hello, world!\n").unwrap(); // 输出字符串
    output.flush().unwrap(); // 刷新缓冲区
}

// rustc main.rs
// ./main
// rm ./main
