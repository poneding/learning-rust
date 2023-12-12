fn main() {
    if_1();
    if_2();
    match_1();
    match_2();

    for_1();

    while_1();

    loop_1();
}

fn if_1() {
    let price = 100;
    if price < 199 && price > 99 {
        println!("buy it!");
    }
}

fn if_2() {
    let score = 70;
    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 60 {
        println!("C");
    } else {
        println!("D");
    }
}

fn match_1() {
    let name = "Jay";
    match name {
        "Jay" | "Ding" => {
            let mut icon_name = String::from("* ");
            icon_name.push_str(name);
            println!("{}", icon_name)
        }
        _ => println!("not matched!"),
    }
}

fn match_2() {
    let score = 79;
    match score {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        60..=79 => println!("C"), // C
        _ => println!("D"),
    }
}

fn for_1() {
    for num in 1..5 {
        println!("** num is {}", num)
    }

    for num in 1..=5 {
        println!("-- num is {}", num)
    }

    let strs = vec!["Hello", "World", "Jay Chou"];
    for str in strs.iter() {
        match str {
            &"Hello" => {
                println!("Yes, Hello");
            }
            _ => {
                println!("item is {}", str)
            }
        }
    }

    let strs2 = vec!["Hello", "World", "Jay Chou"];
    for str in strs2.into_iter() {
        // 每次迭代都会消耗掉元素，也就是被移除
        match str {
            "Hello" => {
                println!("Yes, Hello");
            }
            _ => {
                println!("item is {}", str)
            }
        }
    }

    let mut strs3 = vec!["Hello", "World", "Jay Chou"];
    for str in strs3.iter_mut() {
        // 允许修改集合元素，集合本身必须也要是mut的
        *str = match str {
            &mut "Hello" => "mut Hello",
            _ => *str,
        }
    }
    println!("strs3: {:?}", strs3)
}

fn while_1() {
    let mut num = 1;
    while num <= 10 {
        println!("num is {}", num);
        num = num * 2
    }
}

fn loop_1() {
    let mut num = 1;
    loop {
        if num > 20 {
            break;
        }
        println!("num is {}", num);
        num = num * 3;
    }
}
