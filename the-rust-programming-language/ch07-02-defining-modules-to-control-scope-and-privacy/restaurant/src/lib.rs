#[allow(unused)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

#[allow(unused)]
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用 use 关键字将路径引入作用域
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    // use 还可以一直指定到函数成员或其他成员
    use crate::front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// 但是 use 只适用于其所在的作用域
// 所以下面这段无法编译，因为子模块无法使用到 use 所在的作用域
// use crate::front_of_house::hosting;
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// 使用 pub use 重新导入
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant_2() {
    // 引入本包后可以直接使用 crate::hosting::add_to_waitlist()
    crate::hosting::add_to_waitlist();
}

#[allow(unused)]
fn deliver_order() {}

#[allow(unused)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // deliver_order(); // 无法找到deliver_order函数
        // super::deliver_order(); // 使用 super 进入父模块

        // 或者使用 use 引入
        use crate::deliver_order;
        deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String, // 应季蔬菜
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 如果声明 enum 是 pub 的，那么枚举的所有成员都是公开的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 引入标准库中的成员
use std::collections::HashMap;
fn _main_2() {
    // Vec 被 prelude 所以无需显示引入
    let mut vec = Vec::new();
    vec.insert(0, "hello world");
    let mut map = HashMap::new();
    map.insert("hello", "world");
}

// 引入名称冲突，使用 as 关键字
// use std::fmt::Result;
// use std::io::Result;
// fn function() -> Result {} // 无法准确 Result 类型

use std::fmt::Result;
use std::io::Result as IOResult;
fn _function1() -> Result {
    Ok(())
}
fn _function2() -> IOResult<()> {
    Ok(())
}

// 引入外部包
// 需要添加外部库 crate add rand
use rand::Rng;
fn _main_3() {
    let _secret_number = rand::thread_rng().gen_range(1..101);
}
