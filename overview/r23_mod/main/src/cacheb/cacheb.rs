use std::collections::HashMap;
pub struct CacheB {
    m: HashMap<String, String>,
}

impl CacheB {
    pub fn new() -> CacheB {
        CacheB { m: HashMap::new() }
    }

    #[allow(unused)]
    pub fn get(&self, key: &str) -> &str {
        match self.m.get(key) {
            Some(v) => v.as_str(),
            None => "",
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.m.insert(key.into(), value.into());
    }
}
