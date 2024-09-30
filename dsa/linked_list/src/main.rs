use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);

    println!("{:?}", list);

    if let Some(val) = list.pop_front() {
        println!("Popped value: {}", val);
    }
}
