use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);

    println!("{:?}", set); // 无序
}
