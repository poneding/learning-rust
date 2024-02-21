fn main() {
    let u = User {
        Name: String::from("Jay"),
        Age: 18,
    };
    println!("u: {:?}", u);
    println!("u.Name: {}, u.Age: {}", u.Name, u.Age);

    // 默认结构体变量也是不可修改的
    let mut u2 = User {
        Name: String::from("Ding"),
        Age: 30,
    };
    u2.Name = String::from("Ding Peng");
    u2.Age = 31;
    println!("u2: {:?}", u2);

    let u3 = get_user(String::from("Jack"), 45);
    show_user(u3);

    let u4 = get_user(String::from("Ding"), 30);
    let u4_age = u4.get_age();
    println!("u4_age: {}", u4_age);

    // todo:
    //
    let u4_name = u4.get_name();
    println!("u4_name: {}", u4_name);

    // 调用静态方法
    let u5 = User::get_user_static(String::from("Ding Peng"), 30);
    println!("{:?}", u5);

    unit_type_struct();
}

// 元组结构体
struct Pair(String, i32);

// 如果要使用 {:?} 格式化输出，需要在结构体名称前加上 #[derive(Debug)]。
#[derive(Debug)]
struct User {
    Name: String,
    Age: i32,
}

// 结构体做参数
fn show_user(u: User) {
    println!("{:?}", u);
}

// 结构体作为返回值
fn get_user(name: String, age: i32) -> User {
    return User {
        Name: name,
        Age: age,
    };
}

// 结构体的方法
impl User {
    // &self 表示当前结构体的实例。
    // &self 也是结构体普通方法固定的第一个参数，其他参数可选。
    fn get_age(&self) -> i32 {
        return self.Age;
    }
    fn get_name(&self) -> &str {
        // 报错: value borrowed here after move
        return self.Name.as_str();
    }
}

// 结构体静态方法
impl User {
    fn get_user_static(name: String, age: i32) -> User {
        return User {
            Name: name,
            Age: age,
        };
    }
}

// 单元结构体
// unit type 是一个类型，有且仅有一个值，即 ()。
fn unit_type_struct() {
    let pair = (String::from("hello"), 30);
    println!("pair.0: {:?}, pair.1: {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let (name, age) = pair;
    println!("name: {:?}, age: {:?}", name, age);
}
