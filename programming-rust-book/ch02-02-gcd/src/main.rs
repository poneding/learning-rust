use std::io::Write;
use std::str::FromStr;

mod main_v1;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = main_v1::_gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
