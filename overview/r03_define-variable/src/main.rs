#[allow(unused)]
fn main() {
    // 1. 变量区分大小写
    // 2. 必须以字母或下划线开头，不能以数字开头
    // 3. 变量名区分大小写

    // 不可变变量
    let price = 99;
    // 可变变量
    let mut price2 = 199;
    price2 = 179;
    println!("price2 is {}", price2);

    const_var();

    hide_var();
}

fn const_var() {
    // 常量
    // const： 不可改变的值，定义常量时必须指定类型
    // static： 具有生命周期的，是可以改变（须使用static mut 关键字）的变量，
    // 有个特例就是 “string” 字面量。它可以不经改动就被赋给一个 static 变量，
    // 因为它 的类型标记：&’static str 就包含了所要求的生命周期 ‘static。
    // 其他的引用类型都 必须特地声明，使之拥有’static 生命周期。
    const PI: f64 = 3.1415926;
    println!("PI is {}", PI);

    const NAME: &str = "Jay Chou";
    println!("NAME is {}", NAME);

    static_var();
}

fn static_var() {
    static NAME: &'static str = "Ding Peng";
    println!("static NAME is {}", NAME);

    // static NAME2:&'static mut str="NAME_2";
    // NAME2="NAME_22";
    // println!("NAME2 is {}",NAME2)
}

#[allow(unused)]
fn hide_var() {
    // 变量的隐藏
    let age = 10;
    let age = 20;
    // 改变变量值
    println!("age is {}", age);

    // 还可以改变变量数据类型
    let age = "30";
    println!("age is {}", age);
}
