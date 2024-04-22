use core::panic;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let path = "hello.txt";
    // 打开文件
    // let f = File::open(&path); // Result<File, Error>
    // let _ = match f {
    //     Ok(file) => file,
    //     // Err(error) => panic!("Problem opening the file: {:?}", error),

    //     // 错误类型
    //     Err(error) => match error.kind() {
    //         // 文件不存在则创建
    //         ErrorKind::NotFound => match File::create(&path) {
    //             Ok(created_file) => created_file,
    //             Err(other_error) => panic!("Problem creating the file: {:?}", other_error),
    //         },
    //         // 其他错误，打印
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // // 删除文件
    // fs::remove_file(path).expect("Problem removing the file")

    let _ = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            // File::create(path).unwrap_or_else(|create_error| {
            //     panic!("Problem creating the file: {:?}", create_error);
            // });

            File::create(path).expect("Problem creating the file")
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// 问号操作符只能用于返回 Result 的函数
// main 函数也可以返回 Result 类型
#[allow(dead_code)]
fn main2() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::read_to_string("hello.txt")?;

    Ok(())
}
