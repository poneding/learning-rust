use std::fmt::{Display, Formatter};

fn main() {
    let wangcai = Dog {
        name: String::from("旺财"),
    };

    // wangcai.say_hello();
    // wangcai.say_default();

    let kun = Chick {
        name: String::from("坤坤"),
    };
    // kun.say_default();

    // 将 trait 作为参数传递
    // greet(&wangcai);
    // greet(&kun);

    // greet2(&wangcai);
    // greet2(&kun);

    greet3(&wangcai);
    greet3(&kun);

    // trait 作为返回值
    get_greeter().say_hello()
}

#[allow(unused)]
fn greet(greeter: &impl Greeter) {
    greeter.say_hello();
}

// 还记得 largest 函数吗? fn largest<T: PartialOrd + Copy>(list: &[T]) -> T
#[allow(unused)]
fn greet2<T: Greeter>(greeter: &T) {
    greeter.say_hello();
}

// 使用 where 关键字简化，当出现多个泛型参数时，where 更加清晰
#[allow(unused)]
fn greet3<T>(greeter: &T)
where
    T: Greeter,
{
    greeter.say_hello();
}

fn get_greeter() -> impl Greeter {
    Dog::new("旺财 2 号")
}

// Trait 特征，定义共享的方法签名

pub trait Greeter {
    fn say_hello(&self);

    // 默认实现
    fn say_default(&self) {
        println!("哈喽");
    }
}

pub struct Dog {
    pub name: String,
}

impl Dog {
    fn new(name: &str) -> Dog {
        Dog {
            name: String::from(name),
        }
    }
}

// 实现 Greeter 特征
impl Greeter for Dog {
    fn say_hello(&self) {
        println!("{}：汪汪～", self.name);
    }
    // 调用 say_default 时，会调用 trait 中的默认实现
}

pub struct Chick {
    pub name: String,
}

impl Greeter for Chick {
    fn say_hello(&self) {
        println!("{}：你干嘛～", self.name);
    }

    fn say_default(&self) {
        println!("{}：鸡你太美～", self.name);
    }
}

impl Display for Chick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chick: {}", self.name)
    }
}

impl PartialOrd for Chick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
