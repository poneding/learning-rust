// 返回闭包函数
fn make_trim_hello(answer: String) -> impl Fn(&str) -> bool {
    move |challenge| challenge == answer.trim_start_matches("Hello, ")
}

// 对函数某些参数的引用移动到它返回的闭包中
fn make_trim_hello1<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| challenge == answer.trim_start_matches("Hello, ")
}

// 省略生命周期泛型
fn make_trim_hello2(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| challenge == answer.trim_start_matches("Hello, ")
}

fn main() {
    let trim_hello = make_trim_hello("Hello, World".into());
    println!("{}", trim_hello("********")); // false
    println!("{}", trim_hello("World")); // true

    let trim_hello1 = make_trim_hello1("Hello, World");
    println!("{}", trim_hello1("********")); // false
    println!("{}", trim_hello1("World")); // true

    let trim_hello2 = make_trim_hello2("Hello, World");
    println!("{}", trim_hello2("********")); // false
    println!("{}", trim_hello2("World")); // true
}
