fn main() {
    println!("Hello, world!");
}

pub fn hello() -> &'static str {
    "Hello, World!"
}

#[test]
fn hello_world() {
    assert_eq!("Hello, World!", hello());
}
