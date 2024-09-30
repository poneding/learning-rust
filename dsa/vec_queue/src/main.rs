use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);

    println!("{:?}", queue); // [0, 1, 2]

    if let Some(value) = queue.pop_front() {
        println!("Value of front: {}", value); // Value of front: 0
    }
}
