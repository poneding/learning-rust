use minigrep::Config;
use std::{env, process};

// cargo run -- rust test.txt
// IGNORE_CASE=1 cargo run -- rust test.txt
// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(&args).unwrap_or_else(|err| {
//         // print 将内容打印到标准输出流
//         // eprint 将内容打印到标准错误流
//         // println!("{err}");
//         eprintln!("{err}");
//         process::exit(1);
//     });

//     if let Err(err) = minigrep::run(config) {
//         // println!("{err}");
//         eprintln!("{err}");
//         process::exit(1);
//     }
// }

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // print 将内容打印到标准输出流
        // eprint 将内容打印到标准错误流
        // println!("{err}");
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        // println!("{err}");
        eprintln!("{err}");
        process::exit(1);
    }
}
