use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    set.insert(1);
    set.insert(2);

    println!("{:?}", set); // {1, 2} // 有序
}
