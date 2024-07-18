use std::{thread, time::Duration};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    };

    let up1 = Some(ShirtColor::Blue);
    let g1 = store.giveaway(up1);
    println!("The user with preference {:?} gets {:?}", up1, g1);
    assert_eq!(g1, ShirtColor::Blue);

    let up2 = None;
    let g2 = store.giveaway(up2);
    println!("The user with preference {:?} gets {:?}", up2, g2);
    assert_eq!(g2, ShirtColor::Red);

    // 闭包推断类型
    // let _expensive_closure = |num: u32| -> u32 {
    let _expensive_closure = |num| -> u32 {
        println!("claculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // 函数类型推断
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let result_v1 = add_one_v1(1);
    println!("result_v1: {result_v1}");

    // let add_one_v2 = |x: u32| -> u32 { x + 1 };

    let add_one_v4 = |x| x + 1;
    let result_v4 = add_one_v4(1);
    println!("result_v4: {result_v4}");

    let example_closure = |x| x;
    let _s = example_closure(String::from("Hello"));
    // 推断闭包 x 类型为 String，所以下面的一行会报错
    // let n = example_closure(5); // 提示参数类型错误

    // 闭包捕获引用
    // 1. 不可变引用
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // 2. 可变引用
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    // 闭包移动所有权
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // 另外一个线程修改 list
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            _height: 1,
        },
        Rectangle {
            width: 3,
            _height: 5,
        },
        Rectangle {
            width: 7,
            _height: 12,
        },
    ];

    // list.sort_by_key(|r| r.width);
    // println!("{list:?}");

    let mut num_sort_ops = 0;
    list.sort_by_key(|r| {
        num_sort_ops += 1;
        r.width
    });
    println!("{list:?}, sorted in {num_sort_ops} operations");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

// 衬衫
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

// 库存
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // 发送衬衫，如果用户有偏好，就发偏好的，否则从库存中发颜色最多的
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut blue_num = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_num += 1,
                ShirtColor::Blue => blue_num += 1,
            }
        }

        if red_num > blue_num {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
