mod bool;
mod char;
mod float;
mod int;

fn main() {
    let name = "Jay"; // string
    let name2: &str = "Ding";
    let price = 999; // i32
    let checked = true; // bool

    println!("Hello, {}", name);
    println!("Hello, {}", name2);
    println!("price is {}", price);
    println!("checked is {}", checked);

    int::int();
    float::float();
    bool::bool();
    char::char();
}
