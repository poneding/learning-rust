use std::io::stdin;

fn main() {
    println!("What is 3+2? Type your answer and press enter.");
    let mut answer = String::new();
    stdin()
        .read_line(&mut answer)
        .expect("Unable to read standard input");

    if answer == "5" {
        println!("You are correct!")
    } else {
        println!("You are wrong!")
    }

    // 如果你输入 5 并回车，程序会输出 "You are wrong!"，为什么呢？
    // 因为 read_line() 会读取输入的字符串，包括换行符，所以 answer 的值是 "5\r\n"，而不是 "5"

    println!("{:#?}", answer); // 让我们看看 answer 的值是什么，输出 "5\r\n"
                               // 如果基于 Unix 系统，输出 "5\n"

    println!("Type your answer and press enter again.");
    let mut answer = String::new();
    stdin()
        .read_line(&mut answer)
        .expect("Unable to read standard input");
    let answer = answer.trim();
    // trim your input
    if answer == "5" {
        println!("You are correct!")
    } else {
        println!("You are wrong!")
    }
    println!("{:#?}", answer);
}
