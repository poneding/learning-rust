/// 高级函数与闭包
#[allow(unused)]
fn main() {
    let answer = do_twice(add_one, 5);
    println!("the answer is {:?}", answer); // 12

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    let add_one_closure = returns_closure();
    let add_result = add_one_closure(5);
    println!("add_result: {:?}", add_result); // 6
}

// 函数指针
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 三个闭包 trait：Fn, FnMut, FnOnce

// 返回闭包
// fn returns_closure() -> dyn Fn(i32) -> i32 { // 不能直接返回 dyn Fn(i32) -> i32，因为不知道返回值类型的大小（Sized trait）
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
