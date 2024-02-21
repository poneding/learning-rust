use std::fs::File;

fn main() {
    // err1();
    // err2();
    // err3();
    // err_unwrap();
    err_expect();
}

// - 不可恢复的错误 panic!()
fn err1() {
    panic!("err1");
    println!("Hello in err1") // 将不会打印 不可恢复
}

fn err2() {
    let arr = vec![1, 2, 3];
    println!("{}", arr[4]); // 超出索引界限，将会 panic! 不可恢复
    println!("Hello in err2") // 将不会打印
}

// - 可恢复的异常
fn err3() {
    let f = File::open("hello.log"); // 文件不存在，返回值 Result.Err
    println!("{:?}", f);
    println!("Hello in err3") // 将正常打印
}

// unwrap(&self):T
// 是 Result<T,E> 的方法，如果 Result 为 Ok，则返回 Ok 的值
// 如果 Result 是 Err，会 panic!()
fn err_unwrap() {
    let r1 = is_even(2).unwrap();
    println!("r1: {}", r1);

    let r2 = is_even(1).unwrap(); // panic， 后面不运行
    println!("r2: {}", r2);
}

// expect(&self, msg:&str):T
//
// 可以自定义错误信息，方便定位错误
fn err_expect() {
    let r = File::open("hello.log").expect("文件不存在");
    println!("r: {:?}", r);
}

fn is_even(i: i32) -> Result<bool, String> {
    return if i % 2 == 0 {
        Ok(true)
    } else {
        Err("not even".to_string())
    };
}
