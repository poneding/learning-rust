use std::collections::VecDeque;

fn main() {
    let mut queue = Queue::new();
    queue.push(1);
    queue.push(2);

    if let Some(element) = queue.pop() {
        println!("{}", element); // 1
    }
}

struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            elements: VecDeque::new(),
        }
    }

    fn push(&mut self, element: T) {
        self.elements.push_back(element);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop_front()
    }
}
