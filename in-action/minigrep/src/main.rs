use std::{env, process};

use minigrep::Config;
// cargo run -- <search_string> <search_file>
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("{e}");
        process::exit(1);
    }
}
