use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    open_file();
    create_file();
    remove_file();
    append_file();
    write_all_to_file();
    read_file();
}

// Rust 对结构体 File 的操作方法都会返回一个 Result 枚举
// std::fs::File::open() 只读模式打开文件
// std::fs::File::create() 可写方式打开文件，如果文件存在，清空文件；如果文件不存在则新建
// std::fs::remove_file() 删除文件
// std::fs::OpenOptions::append() 追加模式打开文件，如果文件存在，则在文件末尾追加内容；如果文件不存在则新建
// std::io.Writes::write_all() 将 buf 中所有美容写入到输出流
// std::io.Read::read_to_string() 读取所有内容转换为字符串后追加到 buf 中

fn open_file() {
    let f = File::open("hello.log").unwrap();
    println!("file opened: {:?}", f);

    // 文件不存在会失败
    // let f2 = File::open("hello2.log").unwrap();
    // println!("file not existed: {:?}", f2);
}

fn create_file() {
    let f = File::create("create.log").expect("create file failed");
    println!("create file done. {:?}", f);

    let f2 = File::create("hello.log").expect("create file failed");
    println!("create file done. {:?}", f2);
}

fn remove_file() {
    let f = fs::remove_file("create.log");
    if f.is_err() {
        println!("remove file failed: {:?}", f);
    } else {
        println!("remove file success");
    }
    println!("remove file done.");
}

fn append_file() {
    let mut f = OpenOptions::new()
        .append(true)
        .open("hello.log")
        .expect("append file failed");
    f.write(b"Hello Ding!").expect("write failed");
    println!("append file done.");
}

fn write_all_to_file() {
    let mut f = File::create("lang.log").expect("create file failed");
    f.write_all(b"Hello Golang!\n").expect("write failed");
    f.write_all(b"Hello Rust!\n").expect("write failed");
    println!("write all done.");
}

fn read_file() {
    let mut f = File::open("hello.log").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    println!("read file: {}", contents);
}
