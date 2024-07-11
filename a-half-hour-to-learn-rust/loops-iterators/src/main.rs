fn main() {
    // vector
    for i in vec![52, 49, 21] {
        println!("I like the number {i}");
    }

    // slices
    for i in &[52, 49, 21] {
        println!("I like the number {i}")
    }

    // actual iterator
    for c in "Hello, Rust!".chars() {
        println!("Give me a {c}");
    }

    // filter, map
    for c in "Hello, World!"
        .chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase())
    {
        print!("{c}");
    }
    // ELLOORLD

    println!();
}
