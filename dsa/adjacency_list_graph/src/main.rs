use std::collections::HashMap;

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 4);

    println!("{:?}", graph);
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: u32, to: u32) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
        self.edges.entry(to).or_insert(Vec::new()).push(from);
    }
}
