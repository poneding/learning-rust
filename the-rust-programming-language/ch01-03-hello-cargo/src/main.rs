use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    my_println("Hello world from my_println");
    my_println("Hello world2 from my_println");
}

fn my_println(line: &str) {
    let mut output = io::stdout();
    let line = format!("{line}\n");
    output.write(line.as_bytes()).unwrap();
    output.flush().unwrap();
}

// cargo run
