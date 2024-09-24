use std::env::{self};

use text_colorizer::Colorize;

// cargo run World Rust hello hello.out
fn main() {
    let args = parse_args();
    println!("{:?}", args);
}

#[derive(Debug)] //{:?}
struct Arguments {
    target: String,      // 要搜索的字符串，或正则表达式
    replacement: String, // 要替换的字符串，或正则表达式
    filename: String,    // 输入文件名
    output: String,      // 输出文件名
}

fn print_useage() {
    eprintln!(
        "{} - change occurrences of one string into another.",
        "quickreplace".green()
    );

    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_useage();

        eprintln!(
            "{} wrong number of argument: expect 4, got {}.",
            "Error".red().bold(),
            args.len()
        );

        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}
