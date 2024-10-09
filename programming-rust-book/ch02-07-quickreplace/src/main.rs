use std::{
    env::{self},
    fs,
};

use text_colorizer::Colorize;

// cargo run World Rust hello hello.out
fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_data = match regex_replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.output,
                e
            );
            std::process::exit(1);
        }
    }
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

fn regex_replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    // match regex::Regex::new(target) {
    //     Ok(rgx) => Ok(rgx.replace_all(text, replacement).to_string()),
    //     Err(e) => Err(e),
    // }

    // 等效于
    let rgx = regex::Regex::new(target)?;
    Ok(rgx.replace_all(text, replacement).to_string())
}
