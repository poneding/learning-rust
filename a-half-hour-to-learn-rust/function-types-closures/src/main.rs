fn main() {
    for_each_planet(|planet| println!("Hello, {planet}"));

    // 借用规则也适用
    let greeting = String::from("Hello");
    // 闭包借用了 greeting
    for_each_planet(|planet| println!("{greeting}, {planet}"));

    // 借用规则也适用
    let greeting = String::from("Hello");
    // 闭包生命周期比 greeting 的生命周期长，下面的闭包借用了 greeting，闭包不拥有所有权
    // for_each_planet2(|planet| println!("{greeting}, {planet}"));
    // 闭包转移了 greeting 的所有权，后续 greeting 不可用
    for_each_planet2(move |planet| println!("{greeting}, {planet}"));

    foobar(|x| x * 2); // 8
    foobar2(|x: i32| x * 2); // 8

    // 为什么需要可变函数 FnMut？
    // 因为某些闭包可变的借用局部变量
    let mut acc = 2;
    foobar3(|x| {
        acc += 1;
        x * acc
    }) // 24
}

// 闭包是一些捕获上下文的 Fn，FnMut 或 FnOnce 类型的函数
fn for_each_planet<F>(f: F)
where
    F: Fn(&'static str),
{
    f("Earth");
    f("Mars");
    f("Jupiter"); // 木星
}

fn for_each_planet2<F>(f: F)
where
    F: Fn(&'static str) + 'static,
{
    f("Earth");
    f("Mars");
    f("Jupiter"); // 木星
}

//
fn foobar<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    println!("{}", f(f(2)))
}

fn foobar2<F>(mut f: F)
where
    F: FnMut(i32) -> i32,
{
    // 不能重复借用可变函数变量
    // println!("{}", f(f(2)))

    // 正确的用法：
    let tmp = f(2);
    println!("{}", f(tmp));
}

fn foobar3<F>(mut f: F)
where
    F: FnMut(i32) -> i32,
{
    let tmp = f(2);
    println!("{}", f(tmp));
}
