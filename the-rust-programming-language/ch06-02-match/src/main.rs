// use std::cmp::max;
// use std::cmp::min;

// use std::cmp::{max, min};

// use std::{cmp::max, cmp::min};

// match 控制流
fn main() {
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    value_of_coin(alabama_quarter);

    let n_5 = Some(5);
    let n_6 = plus_one(n_5);
    println!("n_6: {}", n_6.unwrap());

    // 通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),    // 得到新奇的帽子
        7 => remove_fancy_hat(), // 失去新奇的帽子
        // other => _move_player(other), // 移动
        // _ => _reroll(), // 重新投掷
        _ => (), // 无事发生
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn _move_player(_other: i32) {}
fn _reroll() {}

#[allow(unused)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
enum Coin {
    Penny,  // 1
    Nickel, // 5
    Dime,   // 10
    // Quarter, // 25
    Quarter(UsState),
}

fn value_of_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // 必须覆盖所有可能的情况
        Some(i) => Some(i + 1),
        None => None,
    }
}
