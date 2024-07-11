fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(1);
    ac.add(2);
    ac.add(3);

    println!("average: {}", ac.average());
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// 封装结构，只公开外部需要的函数
impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    }
}
