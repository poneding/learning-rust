#[derive(Clone)]
pub struct CircularBuffer<T: Clone> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize, // 我们假设 push 从 head 处添加
    tail: usize, // 我们假设 pop 从 tail 处弹出
    full: bool,
}

impl<T: Clone> CircularBuffer<T> {
    // 创建一个新的循环缓冲区
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; capacity],
            capacity,
            head: 0,
            tail: 0,
            full: false,
        }
    }

    // 向缓冲区中添加元素，必要时覆盖旧元素
    pub fn push(&mut self, item: T) {
        self.buffer[self.head] = Some(item);
        self.head = (self.head + 1) % self.capacity;

        // 如果缓冲区满了，则需要移动 tail 指针
        if self.full {
            self.tail = (self.tail + 1) % self.capacity;
        } else {
            // 如果 head 追上了 tail
            self.full = self.head == self.tail;
        }
    }

    // 从缓冲区中弹出元素
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.tail].take();
        self.tail = (self.tail + 1) % self.capacity;
        self.full = false; // 一旦弹出元素，缓冲区就不再满了

        item
    }

    // 检查缓冲区是否为空
    pub fn is_empty(&self) -> bool {
        !self.full && (self.head == self.tail)
    }

    // 检查缓冲区是否已满
    pub fn is_full(&self) -> bool {
        self.full
    }

    // 获取缓冲区中元素的数量
    pub fn len(&self) -> usize {
        if self.full {
            self.capacity
        } else if self.head >= self.tail {
            self.head - self.tail
        } else {
            self.capacity - self.tail + self.head
        }
    }

    // 获取缓冲区的容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

fn main() {
    let mut buffer = CircularBuffer::new(3);

    buffer.push(1);
    buffer.push(2);
    buffer.push(3);

    println!("Buffer full: {}", buffer.is_full());

    // 添加第四个元素，会覆盖掉最旧的元素
    buffer.push(4);

    println!("Pop: {:?}", buffer.pop()); // 应该弹出 2（因为 1 已被覆盖）
    println!("Pop: {:?}", buffer.pop()); // 应该弹出 3
    println!("Pop: {:?}", buffer.pop()); // 应该弹出 4
    println!("Buffer empty: {}", buffer.is_empty());
    buffer.push(5);
    buffer.push(6);
    buffer.push(7);
    buffer.push(8);

    println!("Pop: {:?}", buffer.pop()); // 应该弹出 6（因为 5 已被覆盖）
    println!("Pop: {:?}", buffer.pop()); // 应该弹出 7
    println!("Pop: {:?}", buffer.pop()); // 应该弹出 8
}
