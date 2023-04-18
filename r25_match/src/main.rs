fn main() {
    match1();
    match2();
}

// 解引用使用 *
// 解构使用 &，ref 和 mut ref
fn match1() {
    let num = &100;
    match num {
        &val => println!("val-1: {}", val),
    }
    match *num {
        val => println!("val-2: {}", val),
    }

    // 使用 ref 更改赋值行为，从而对具体值创建引用
    let ref a = 1;
    println!("a: {}", a);

    // 定义非引用的变量，可以通过使用 ref 和 ref mut 取其引用
    let b = 2;
    let mut c = 3;
    match b {
        ref mb => {
            println!("mb: {}", mb);
            println!("*mb: {}", *mb); // 因为 mb 是一个引用，所以可以使用 *mb 获取其值
        }
    }

    match c {
        ref mut mvc => {
            *mvc *= *mvc;
            println!("mvc: {}", mvc);
            println!("*mvc: {}", *mvc);
        }
    }
}

// 结构结构体
struct User {
    name: String,
    age: u8,
}

fn match2() {
    let u1 = User {
        name: "Ding".to_string(),
        age: 18,
    };
    let User {
        // name: name,
        // age: age
        name,   //如果同名可以简写
        age,
    } = u1;
    println!("name: {}, age: {}", name, age);

    // 忽略不需要的字段
    let u2 = User {
        name: "Jay".to_string(),
        age: 30,

    };
    let User {
        name,
        ..
    } = u2;
    println!("name: {}", name)
}
