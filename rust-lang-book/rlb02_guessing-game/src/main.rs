use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    // println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..101); // (1, 101) 取值范围是 [1, 101)，不包括 101，也可以写成 (1..=100)

    println!("The secret number is: {}", secret_number); // debug 时打印出来

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // mut 可变变量
        io::stdin()
            .read_line(&mut guess) // & 表示引用，引用是不可变的，所以要加上 mut
            .expect("Failed to read line"); // expect 是 io::Result 的一个方法，如果 Result 是 Err，expect 会使程序崩溃，并显示传递给它的信息

        // match guess.trim().parse::<u32>() {
        //     Ok(num) => {
        //         println!("You guessed: {}", num);
        //         if num < secret_number {
        //             println!("Too small!");
        //         } else if num > secret_number {
        //             println!("Too big!");
        //         } else {
        //             println!("You win!");
        //             break;
        //         }
        //     }
        //     Err(_) => {
        //         println!("Please input a number!");
        //         continue;
        //     }
        // }

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
