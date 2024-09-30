use std::collections::HashMap;

fn main() {
    let mut tire = Tire::new();
    tire.insert("hello");
    tire.insert("world");

    println!("{:?}", tire);
}

#[derive(Debug)]
struct TireNode {
    children: HashMap<char, TireNode>,
    end_of_word: bool,
}

impl TireNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            end_of_word: false,
        }
    }
}

#[derive(Debug)]
struct Tire {
    root: TireNode,
}

impl Tire {
    fn new() -> Self {
        Self {
            root: TireNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        for c in word.chars() {
            self.root.children.entry(c).or_insert(TireNode::new());
        }
        self.root.end_of_word = true;
    }
}
