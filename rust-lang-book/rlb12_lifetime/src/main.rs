fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Jay Chou");

    let result = longest(&s1, &s2);
    println!("The longest string is {}", result);
}

// &str 一个引用
// &'a str 一个带有显示生命周期注解的引用
// &'a mut str 一个带有显示生命周期注解的可变引用
// 返回值的生命周期会是参数的生命周期中较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
