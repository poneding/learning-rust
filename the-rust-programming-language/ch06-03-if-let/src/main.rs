// if let 控制流

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        // None => (),
        _ => (),
    }

    // 等价于
    if let Some(max) = config_max {
        println!("The maximumu is configured to be {max}");
    }

    enum Coin {
        _Penny,
        Quarter(String),
    }

    let mut _count = 0;
    let coin = Coin::Quarter("Alabama".to_string());
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}"),
    //     _ => count += 1,
    // }

    // 等价于
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}")
    } else {
        _count += 1;
    }
}
