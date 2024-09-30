use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(3);
    heap.push(2);

    if let Some(value) = heap.pop() {
        println!("Popped value: {}", value); // Popped value: 3
    }
}
