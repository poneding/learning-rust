use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..101); // (1, 101) 取值范围是 [1, 101)，不包括 101，也可以写成 (1..=100)

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
