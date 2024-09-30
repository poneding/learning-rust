fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);

    if let Some(element) = stack.pop() {
        println!("{}", element); // 2
    }
}

struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}
