use core::panic;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn _main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Failed to create the file: {err:?}"),
            },
            other_err => panic!("Failed to open the file: {other_err:?}"),
        },
    };

    let _greeting_file = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|err| panic!("Failed to create the file: {err:?}"))
        } else {
            panic!("Failed to open the file: {err:?}");
        }
    });

    let _greeting_file = File::open("hello.txt").unwrap();
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project.");

    if let Ok(username) = read_username_from_file1() {
        println!("username: {username}")
    }

    match read_username_from_file2() {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("{}", e),
    }

    match read_username_from_file3() {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("{}", e),
    }

    match read_username_from_file4() {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("{}", e),
    }
}

// 传播错误
fn read_username_from_file1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 使用 ? 问号运算符来处理，如果 match 到 Err，直接返回，要求函数的返回类型是 Result<T,E> 或 Option<T>
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 更精简，直接在 ? 后面继续调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? 适用于返回 Option<T> 的函数
fn _last_char_of_first_line(text: &str) -> Option<char> {
    // 如果没找到字符，返回 None
    text.lines().next()?.chars().last()
}

// 在 main 函数中使用 ?，需要将 main 函数的返回类型改成 Result<(), Box<dyn Error>>
fn main() -> Result<(), Box<dyn Error>> {
    _main();

    let _greeting_file = File::open("hello1.txt")?;
    Ok(())
}
