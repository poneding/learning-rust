fn main() {
    let mut deque = Deque::new();
    deque.push_front(1);
    deque.push_front(2);
    deque.push_back(3);
    deque.push_back(4);

    if let Some(element) = deque.pop_front() {
        println!("{}", element); // 2
    }

    if let Some(element) = deque.pop_back() {
        println!("{}", element); // 4
    }
}

struct Deque<T> {
    elements: Vec<T>,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    fn push_front(&mut self, element: T) {
        self.elements.insert(0, element);
    }

    fn push_back(&mut self, element: T) {
        self.elements.push(element);
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        self.elements.pop()
    }
}
