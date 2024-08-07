# 猜数游戏

## 创建项目

```bash
cargo new guessing-game
cd guessing-game
```

编写 *src/main.rs* 文件：

```rust
use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

## 编译运行

```bash
cargo run
```

## 引入 rand 包

在 *Cargo.toml* 中添加依赖：

```toml
[dependencies]
rand = "0.8.5"
```

或者，直接使用 `cargo add rand` 命令。

## 使用 rand 包

修改 *src/main.rs* 文件：

```rust
use rand::Rng;
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

        match guess.trim().parse::<u32>() {
            Ok(num) => {
                println!("You guessed: {}", num);
                if num < secret_number {
                    println!("Too small!");
                } else if num > secret_number {
                    println!("Too big!");
                } else {
                    println!("You win!");
                    break;
                }
            }
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        }
    }
}
```

## 再次编译运行

```bash
cargo run
```

可以开始猜数游戏了。
