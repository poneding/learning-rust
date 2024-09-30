use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(1, "a");
    map.insert(3, "c");
    map.insert(2, "b");

    println!("{:?}", map); // {1: "a", 2: "b", 3: "c"} // 有序

    if let Some(value) = map.get(&1) {
        println!("Value of key 1: {}", value); // Value of key 1: a
    }
}
