use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    let mut filter = BloomFilter::new(100, 3);
    filter.insert(&"hello");
    println!("{}", filter.contains(&"hello")); // true
    println!("{}", filter.contains(&"world")); // false
}

struct BloomFilter {
    bitmap: Vec<bool>,
    num_hashes: usize,
}

impl BloomFilter {
    fn new(size: usize, num_hashes: usize) -> Self {
        Self {
            bitmap: vec![false; size],
            num_hashes,
        }
    }

    fn hash<T: Hash>(&self, item: &T, i: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        (hasher.finish() as usize + i * 0x9e779b9) % self.bitmap.len()
    }

    fn insert<T: Hash>(&mut self, item: &T) {
        for i in 0..self.num_hashes {
            let pos = self.hash(item, i);
            self.bitmap[pos] = true;
        }
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.num_hashes {
            let pos = self.hash(item, i);
            if !self.bitmap[pos] {
                return false;
            }
        }
        true
    }
}
